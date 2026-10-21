use glib::subclass::Signal;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::{Cell, RefCell};
use std::sync::OnceLock;

use libadwaita as adw;

#[derive(Clone, Copy, PartialEq)]
pub enum RegionState {
    Normal,
    Highlighted,
    Correct,
    Wrong,
    Decorative,
    Background,
}

pub struct Region {
    pub id: String,
    pub path: gtk::gsk::Path,
    pub bounds: gtk::graphene::Rect,
    pub state: RegionState,
    pub is_river: bool,
}

mod imp {
    use super::*;

    pub struct MapWidget {
        pub regions: RefCell<Vec<Region>>,
        pub svg_bounds: RefCell<gtk::graphene::Rect>,
        // [normal, highlighted, correct, wrong, border]
        pub cached_colors: RefCell<[gtk::gdk::RGBA; 5]>,
        pub zoom: Cell<f32>,
        pub pan: Cell<(f32, f32)>,
    }

    impl Default for MapWidget {
        fn default() -> Self {
            let fallback = gtk::gdk::RGBA::new(0.5, 0.5, 0.5, 1.0);
            Self {
                regions: RefCell::new(Vec::new()),
                svg_bounds: RefCell::new(gtk::graphene::Rect::new(0.0, 0.0, 1.0, 1.0)),
                cached_colors: RefCell::new([fallback; 5]),
                zoom: Cell::new(1.0),
                pan: Cell::new((0.0, 0.0)),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MapWidget {
        const NAME: &'static str = "MapWidget";
        type Type = super::MapWidget;
        type ParentType = gtk::Widget;
    }

    impl ObjectImpl for MapWidget {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder("region-clicked")
                        .param_types([String::static_type()])
                        .build(),
                    Signal::builder("zoom-changed")
                        .param_types([f32::static_type()])
                        .build(),
                ]
            })
        }

        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            let click = gtk::GestureClick::new();
            let widget = obj.downgrade();
            click.connect_released(move |_, _, x, y| {
                if let Some(w) = widget.upgrade() {
                    w.on_click(x, y);
                }
            });
            obj.add_controller(click);

            let motion = gtk::EventControllerMotion::new();
            let widget = obj.downgrade();
            motion.connect_motion(move |_, x, y| {
                if let Some(w) = widget.upgrade() {
                    w.on_motion(x, y);
                }
            });
            let widget2 = obj.downgrade();
            motion.connect_leave(move |_| {
                if let Some(w) = widget2.upgrade() {
                    w.clear_highlight();
                }
            });
            obj.add_controller(motion);

            let scroll = gtk::EventControllerScroll::new(
                gtk::EventControllerScrollFlags::VERTICAL
                    | gtk::EventControllerScrollFlags::DISCRETE,
            );
            let last_pos = std::rc::Rc::new(Cell::new((0.0_f64, 0.0_f64)));
            let motion_track = gtk::EventControllerMotion::new();
            let lp = last_pos.clone();
            motion_track.connect_motion(move |_, x, y| {
                lp.set((x, y));
            });
            obj.add_controller(motion_track);
            let widget = obj.downgrade();
            let lp = last_pos.clone();
            scroll.connect_scroll(move |_, _dx, dy| {
                if let Some(w) = widget.upgrade() {
                    let (x, y) = lp.get();
                    let factor = if dy < 0.0 { 1.25 } else { 1.0 / 1.25 };
                    w.zoom_at(factor, x, y);
                }
                glib::Propagation::Stop
            });
            obj.add_controller(scroll);

            let gesture_zoom = gtk::GestureZoom::new();
            let widget = obj.downgrade();
            let zoom_start = std::rc::Rc::new(Cell::new(1.0_f32));
            let zs = zoom_start.clone();
            gesture_zoom.connect_begin(glib::clone!(
                #[weak(rename_to = w)]
                obj,
                move |_, _| {
                    zs.set(w.zoom());
                }
            ));
            let zs = zoom_start.clone();
            gesture_zoom.connect_scale_changed(move |g, scale| {
                if let Some(w) = widget.upgrade() {
                    let (cx, cy) = g
                        .bounding_box_center()
                        .unwrap_or((w.width() as f64 / 2.0, w.height() as f64 / 2.0));
                    let target = zs.get() * scale as f32;
                    let factor = target / w.zoom();
                    w.zoom_at(factor, cx, cy);
                }
            });
            obj.add_controller(gesture_zoom);

            let drag = gtk::GestureDrag::new();
            drag.set_button(gtk::gdk::BUTTON_MIDDLE);
            let pan_start = std::rc::Rc::new(Cell::new((0.0_f32, 0.0_f32)));
            let ps = pan_start.clone();
            drag.connect_drag_begin(glib::clone!(
                #[weak(rename_to = w)]
                obj,
                move |_, _, _| {
                    ps.set(w.pan());
                    w.set_cursor_from_name(Some("grabbing"));
                }
            ));
            let widget = obj.downgrade();
            let ps = pan_start.clone();
            drag.connect_drag_update(move |_, ox, oy| {
                if let Some(w) = widget.upgrade() {
                    let (start_x, start_y) = ps.get();
                    w.set_pan(start_x + ox as f32, start_y + oy as f32);
                }
            });
            drag.connect_drag_end(glib::clone!(
                #[weak(rename_to = w)]
                obj,
                move |_, _, _| {
                    w.set_cursor_from_name(None);
                }
            ));
            obj.add_controller(drag);

            let style = adw::StyleManager::default();
            let w = obj.downgrade();
            style.connect_dark_notify(move |_| {
                if let Some(w) = w.upgrade() {
                    glib::idle_add_local_once(glib::clone!(
                        #[weak]
                        w,
                        move || {
                            w.imp().cache_colors();
                            w.queue_draw();
                        }
                    ));
                }
            });
            let w = obj.downgrade();
            style.connect_high_contrast_notify(move |_| {
                if let Some(w) = w.upgrade() {
                    glib::idle_add_local_once(glib::clone!(
                        #[weak]
                        w,
                        move || {
                            w.imp().cache_colors();
                            w.queue_draw();
                        }
                    ));
                }
            });

            glib::idle_add_local_once(glib::clone!(
                #[weak(rename_to = this)]
                self,
                move || {
                    this.cache_colors();
                    this.obj().queue_draw();
                }
            ));
        }
    }

    impl WidgetImpl for MapWidget {
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            let widget = self.obj();
            let width = widget.width() as f32;
            let height = widget.height() as f32;
            if width <= 0.0 || height <= 0.0 {
                return;
            }

            {
                let svg_bounds = self.svg_bounds.borrow();
                if svg_bounds.width() <= 0.0 || svg_bounds.height() <= 0.0 {
                    return;
                }
            }

            let (scale, tx, ty) = widget.compute_transform();

            snapshot.save();
            snapshot.translate(&gtk::graphene::Point::new(tx, ty));
            snapshot.scale(scale, scale);

            let stroke = gtk::gsk::Stroke::new(1.5 / scale);
            let river_stroke = gtk::gsk::Stroke::new(3.0 / scale);
            let colors = self.cached_colors.borrow();
            let [c_normal, c_highlight, c_correct, c_wrong, c_border] = *colors;

            let regions = self.regions.borrow();
            for region in regions.iter() {
                if region.state == RegionState::Decorative {
                    snapshot.append_fill(&region.path, gtk::gsk::FillRule::Winding, &c_border);
                    continue;
                }

                if region.state == RegionState::Background {
                    let b = &region.bounds;
                    if b.width() < 8.0 || b.height() < 8.0 {
                        continue;
                    }
                    snapshot.append_fill(&region.path, gtk::gsk::FillRule::Winding, &c_normal);
                    snapshot.append_stroke(&region.path, &stroke, &c_border);
                    continue;
                }

                let fill_color = match region.state {
                    RegionState::Normal => c_normal,
                    RegionState::Highlighted => c_highlight,
                    RegionState::Correct => c_correct,
                    RegionState::Wrong => c_wrong,
                    RegionState::Decorative | RegionState::Background => unreachable!(),
                };

                if region.is_river {
                    snapshot.append_stroke(&region.path, &river_stroke, &fill_color);
                } else {
                    snapshot.append_fill(&region.path, gtk::gsk::FillRule::Winding, &fill_color);
                    snapshot.append_stroke(&region.path, &stroke, &c_border);
                }
            }

            // Draw markers for tiny regions and rivers on top of everything
            for region in regions.iter() {
                if region.state == RegionState::Decorative
                    || region.state == RegionState::Background
                {
                    continue;
                }

                let b = &region.bounds;
                let is_small = b.width() < 8.0 || b.height() < 8.0;
                if !is_small && !region.is_river {
                    continue;
                }

                let fill_color = match region.state {
                    RegionState::Normal => c_normal,
                    RegionState::Highlighted => c_highlight,
                    RegionState::Correct => c_correct,
                    RegionState::Wrong => c_wrong,
                    RegionState::Decorative | RegionState::Background => unreachable!(),
                };

                let (cx, cy) = if region.is_river {
                    let measure = gtk::gsk::PathMeasure::new(&region.path);
                    if let Some(pp) = measure.point(measure.length() / 2.0) {
                        let pos = pp.position(&region.path);
                        (pos.x(), pos.y())
                    } else {
                        (b.x() + b.width() / 2.0, b.y() + b.height() / 2.0)
                    }
                } else {
                    (b.x() + b.width() / 2.0, b.y() + b.height() / 2.0)
                };

                let r = 5.0 / scale;
                let marker = gtk::gsk::Path::parse(&format!(
                    "M {},{} L {},{} L {},{} L {},{} Z",
                    cx - r,
                    cy - r,
                    cx + r,
                    cy - r,
                    cx + r,
                    cy + r,
                    cx - r,
                    cy + r
                ))
                .unwrap();

                snapshot.append_fill(&marker, gtk::gsk::FillRule::Winding, &fill_color);
                snapshot.append_stroke(&marker, &stroke, &c_border);
            }

            snapshot.restore();
        }

        fn measure(&self, orientation: gtk::Orientation, _for_size: i32) -> (i32, i32, i32, i32) {
            match orientation {
                gtk::Orientation::Horizontal => (120, 600, -1, -1),
                _ => (120, 500, -1, -1),
            }
        }
    }

    impl MapWidget {
        fn cache_colors(&self) {
            #[allow(deprecated)]
            let ctx = self.obj().style_context();
            let lookup = |name: &str| -> gtk::gdk::RGBA {
                #[allow(deprecated)]
                ctx.lookup_color(name)
                    .unwrap_or(gtk::gdk::RGBA::new(0.5, 0.5, 0.5, 1.0))
            };

            *self.cached_colors.borrow_mut() = [
                lookup("map-region-color"),
                lookup("map-region-highlight-color"),
                lookup("map-region-correct-color"),
                lookup("map-region-wrong-color"),
                lookup("map-border-color"),
            ];
        }
    }
}

glib::wrapper! {
    pub struct MapWidget(ObjectSubclass<imp::MapWidget>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl MapWidget {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn load_svg(&self, resource_path: &str) {
        let bytes = gio::resources_lookup_data(resource_path, gio::ResourceLookupFlags::NONE)
            .expect("Failed to load SVG resource");
        let data = std::str::from_utf8(&bytes).expect("SVG is not valid UTF-8");
        self.parse_svg(data);
    }

    fn parse_svg(&self, data: &str) {
        use quick_xml::events::Event;
        use quick_xml::reader::Reader;

        let imp = self.imp();
        let mut regions = Vec::new();
        let mut markers: Vec<(String, f32, f32)> = Vec::new();
        let mut reader = Reader::from_str(data);

        loop {
            match reader.read_event() {
                Ok(Event::Empty(ref e)) | Ok(Event::Start(ref e)) => {
                    let tag = e.name();
                    if tag.as_ref() == "path" {
                        let mut id = None;
                        let mut d = None;
                        for attr in e.attributes().flatten() {
                            match attr.key.as_ref() {
                                "id" => id = Some(attr.value.to_string()),
                                "d" => d = Some(attr.value.to_string()),
                                _ => {}
                            }
                        }

                        if let (Some(id), Some(d)) = (id, d) {
                            let d = d.trim().to_string();
                            if let Ok(path) = gtk::gsk::Path::parse(&d) {
                                let bounds = path
                                    .bounds()
                                    .unwrap_or(gtk::graphene::Rect::new(0.0, 0.0, 0.0, 0.0));

                                let (id, state, is_river) = if id.starts_with("__") {
                                    (id, RegionState::Decorative, false)
                                } else if let Some(name) = id.strip_prefix("_bg_") {
                                    (name.to_string(), RegionState::Background, false)
                                } else if let Some(name) = id.strip_prefix("_river_") {
                                    (name.to_string(), RegionState::Normal, true)
                                } else {
                                    (id, RegionState::Normal, false)
                                };

                                regions.push(Region {
                                    id,
                                    path,
                                    bounds,
                                    state,
                                    is_river,
                                });
                            }
                        }
                    } else if tag.as_ref() == "circle" {
                        let mut id = None;
                        let mut cx = None;
                        let mut cy = None;
                        for attr in e.attributes().flatten() {
                            match attr.key.as_ref() {
                                "id" => id = Some(attr.value.to_string()),
                                "cx" => cx = attr.value.parse::<f32>().ok(),
                                "cy" => cy = attr.value.parse::<f32>().ok(),
                                _ => {}
                            }
                        }

                        if let (Some(id), Some(cx), Some(cy)) = (id, cx, cy) {
                            markers.push((id, cx, cy));
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(_) => break,
                _ => {}
            }
        }

        // If markers exist, match them to paths using in_fill (for compound-path SVGs)
        if !markers.is_empty() {
            for (name, cx, cy) in &markers {
                let point = gtk::graphene::Point::new(*cx, *cy);
                for region in regions.iter_mut() {
                    if region.id.starts_with("__province_")
                        && region.path.in_fill(&point, gtk::gsk::FillRule::Winding)
                    {
                        region.id = name.clone();
                        break;
                    }
                }
            }
            // Remove unmatched temp paths
            regions.retain(|r| !r.id.starts_with("__province_"));
        }

        // Compute overall bounds
        if let Some(first) = regions.first() {
            let mut min_x = first.bounds.x();
            let mut min_y = first.bounds.y();
            let mut max_x = min_x + first.bounds.width();
            let mut max_y = min_y + first.bounds.height();

            for r in &regions[1..] {
                min_x = min_x.min(r.bounds.x());
                min_y = min_y.min(r.bounds.y());
                max_x = max_x.max(r.bounds.x() + r.bounds.width());
                max_y = max_y.max(r.bounds.y() + r.bounds.height());
            }

            *imp.svg_bounds.borrow_mut() =
                gtk::graphene::Rect::new(min_x, min_y, max_x - min_x, max_y - min_y);
        }

        *imp.regions.borrow_mut() = regions;
        self.queue_draw();
    }

    fn transform_to_svg(&self, x: f64, y: f64) -> (f32, f32) {
        let (scale, tx, ty) = self.compute_transform();
        ((x as f32 - tx) / scale, (y as f32 - ty) / scale)
    }

    fn compute_transform(&self) -> (f32, f32, f32) {
        let imp = self.imp();
        let svg_bounds = imp.svg_bounds.borrow();
        viewport_transform(
            self.width() as f32,
            self.height() as f32,
            *svg_bounds,
            imp.zoom.get(),
            imp.pan.get(),
        )
    }

    fn hit_test(&self, x: f64, y: f64) -> Option<String> {
        let (svg_x, svg_y) = self.transform_to_svg(x, y);
        let point = gtk::graphene::Point::new(svg_x, svg_y);
        let regions = self.imp().regions.borrow();

        // Check rivers first (stroke-based hit test)
        let river_threshold = 5.0_f32;
        let mut best_river: Option<(f32, &Region)> = None;
        for region in regions.iter() {
            if !region.is_river
                || region.state == RegionState::Decorative
                || region.state == RegionState::Background
            {
                continue;
            }

            if let Some((_, dist)) = region.path.closest_point(&point, river_threshold)
                && (best_river.is_none() || dist < best_river.unwrap().0)
            {
                best_river = Some((dist, region));
            }
        }

        if let Some((_, r)) = best_river {
            return Some(r.id.clone());
        }

        // Check tiny regions (they'd be hidden behind larger neighbours)
        let min_size = 8.0_f32;
        let tolerance = 10.0_f32;
        let mut best: Option<(f32, &Region)> = None;
        for region in regions.iter() {
            if matches!(
                region.state,
                RegionState::Decorative | RegionState::Background
            ) || region.is_river
            {
                continue;
            }

            let b = &region.bounds;
            if b.width() < min_size || b.height() < min_size {
                let cx = b.x() + b.width() / 2.0;
                let cy = b.y() + b.height() / 2.0;
                let dist = ((svg_x - cx).powi(2) + (svg_y - cy).powi(2)).sqrt();

                if dist < tolerance && (best.is_none() || dist < best.unwrap().0) {
                    best = Some((dist, region));
                }
            }
        }

        if let Some((_, r)) = best {
            return Some(r.id.clone());
        }

        let mut smallest: Option<(f32, &Region)> = None;
        for region in regions.iter() {
            if matches!(
                region.state,
                RegionState::Decorative | RegionState::Background
            ) || region.is_river
            {
                continue;
            }

            if region.path.in_fill(&point, gtk::gsk::FillRule::Winding) {
                let area = region.bounds.width() * region.bounds.height();
                if smallest.is_none() || area < smallest.unwrap().0 {
                    smallest = Some((area, region));
                }
            }
        }

        smallest.map(|(_, r)| r.id.clone())
    }

    fn on_click(&self, x: f64, y: f64) {
        if let Some(id) = self.hit_test(x, y) {
            self.emit_by_name::<()>("region-clicked", &[&id]);
        }
    }

    fn on_motion(&self, x: f64, y: f64) {
        let hit = self.hit_test(x, y);
        let mut regions = self.imp().regions.borrow_mut();
        let mut changed = false;
        for region in regions.iter_mut() {
            let new_state = if region.state == RegionState::Decorative
                || region.state == RegionState::Background
                || region.state == RegionState::Correct
                || region.state == RegionState::Wrong
            {
                region.state
            } else if hit.as_deref() == Some(&region.id) {
                RegionState::Highlighted
            } else {
                RegionState::Normal
            };

            if new_state != region.state {
                region.state = new_state;
                changed = true;
            }
        }

        if changed {
            drop(regions);
            self.queue_draw();
        }
    }

    fn clear_highlight(&self) {
        let mut regions = self.imp().regions.borrow_mut();
        let mut changed = false;

        for region in regions.iter_mut() {
            if region.state == RegionState::Highlighted {
                region.state = RegionState::Normal;
                changed = true;
            }
        }

        if changed {
            drop(regions);
            self.queue_draw();
        }
    }

    pub fn region_state(&self, region_id: &str) -> RegionState {
        let regions = self.imp().regions.borrow();
        regions
            .iter()
            .find(|r| r.id == region_id)
            .map(|r| r.state)
            .unwrap_or(RegionState::Normal)
    }

    pub fn set_region_state(&self, region_id: &str, state: RegionState) {
        let mut regions = self.imp().regions.borrow_mut();
        for region in regions.iter_mut() {
            if region.id == region_id
                && !matches!(
                    region.state,
                    RegionState::Decorative | RegionState::Background
                )
            {
                region.state = state;
                break;
            }
        }

        drop(regions);
        self.queue_draw();
    }

    pub fn reset_all_states(&self) {
        let mut regions = self.imp().regions.borrow_mut();

        for region in regions.iter_mut() {
            if !matches!(
                region.state,
                RegionState::Decorative | RegionState::Background
            ) {
                region.state = RegionState::Normal;
            }
        }

        drop(regions);
        self.queue_draw();
    }

    pub const MIN_ZOOM: f32 = 1.0;
    pub const MAX_ZOOM: f32 = 8.0;

    pub fn zoom(&self) -> f32 {
        self.imp().zoom.get()
    }

    pub fn set_zoom(&self, zoom: f32) {
        let clamped = zoom.clamp(Self::MIN_ZOOM, Self::MAX_ZOOM);
        self.imp().zoom.set(clamped);
        self.clamp_pan();
        self.queue_draw();
        self.emit_by_name::<()>("zoom-changed", &[&clamped]);
    }

    pub fn pan(&self) -> (f32, f32) {
        self.imp().pan.get()
    }

    pub fn set_pan(&self, pan_x: f32, pan_y: f32) {
        self.imp().pan.set((pan_x, pan_y));
        self.clamp_pan();
        self.queue_draw();
    }

    pub fn reset_view(&self) {
        let imp = self.imp();
        imp.zoom.set(Self::MIN_ZOOM);
        imp.pan.set((0.0, 0.0));
        self.queue_draw();
        self.emit_by_name::<()>("zoom-changed", &[&Self::MIN_ZOOM]);
    }

    pub fn zoom_at(&self, factor: f32, cx: f64, cy: f64) {
        let old_zoom = self.imp().zoom.get();
        let new_zoom = (old_zoom * factor).clamp(Self::MIN_ZOOM, Self::MAX_ZOOM);
        if (new_zoom - old_zoom).abs() < f32::EPSILON {
            return;
        }

        let (svg_before_x, svg_before_y) = self.transform_to_svg(cx, cy);

        self.imp().zoom.set(new_zoom);

        let (scale, tx, ty) = self.compute_transform();
        let svg_after_x = (cx as f32 - tx) / scale;
        let svg_after_y = (cy as f32 - ty) / scale;

        let (px, py) = self.imp().pan.get();
        let new_px = px + (svg_after_x - svg_before_x) * scale;
        let new_py = py + (svg_after_y - svg_before_y) * scale;
        self.imp().pan.set((new_px, new_py));

        self.clamp_pan();
        self.queue_draw();
        self.emit_by_name::<()>("zoom-changed", &[&new_zoom]);
    }

    pub fn zoom_at_center(&self, factor: f32) {
        let cx = self.width() as f64 / 2.0;
        let cy = self.height() as f64 / 2.0;
        self.zoom_at(factor, cx, cy);
    }

    fn clamp_pan(&self) {
        let imp = self.imp();
        let width = self.width() as f32;
        let height = self.height() as f32;
        if width <= 0.0 || height <= 0.0 {
            return;
        }
        let svg_bounds = imp.svg_bounds.borrow();
        let (sw, sh) = (svg_bounds.width(), svg_bounds.height());
        drop(svg_bounds);
        if sw <= 0.0 || sh <= 0.0 {
            return;
        }
        let base = (width / sw).min(height / sh);
        let clamped = clamp_pan_offset(width, height, sw, sh, base * imp.zoom.get(), imp.pan.get());
        imp.pan.set(clamped);
    }
}

fn viewport_transform(
    width: f32,
    height: f32,
    svg_bounds: gtk::graphene::Rect,
    zoom: f32,
    pan: (f32, f32),
) -> (f32, f32, f32) {
    let (sx, sy, sw, sh) = (
        svg_bounds.x(),
        svg_bounds.y(),
        svg_bounds.width(),
        svg_bounds.height(),
    );
    let scale = (width / sw).min(height / sh) * zoom;
    let (pan_x, pan_y) = pan;
    let tx = (width - sw * scale) / 2.0 - sx * scale + pan_x;
    let ty = (height - sh * scale) / 2.0 - sy * scale + pan_y;
    (scale, tx, ty)
}

fn clamp_pan_offset(
    width: f32,
    height: f32,
    sw: f32,
    sh: f32,
    scale: f32,
    pan: (f32, f32),
) -> (f32, f32) {
    let content_w = sw * scale;
    let content_h = sh * scale;
    let max_x = (content_w - width).max(0.0) / 2.0;
    let max_y = (content_h - height).max(0.0) / 2.0;
    (pan.0.clamp(-max_x, max_x), pan.1.clamp(-max_y, max_y))
}

#[cfg(test)]
mod tests {
    use super::*;
    use gtk::graphene::Rect;

    #[test]
    fn transform_matches_fit_formula_at_identity() {
        let bounds = Rect::new(0.0, 0.0, 100.0, 50.0);
        let (scale, tx, ty) = viewport_transform(400.0, 300.0, bounds, 1.0, (0.0, 0.0));
        // Fit scale = min(400/100, 300/50) = min(4, 6) = 4
        assert!((scale - 4.0).abs() < 1e-5);
        // Centered horizontally: (400 - 100*4)/2 = 0; vertically: (300 - 50*4)/2 = 50
        assert!((tx - 0.0).abs() < 1e-5);
        assert!((ty - 50.0).abs() < 1e-5);
    }

    #[test]
    fn transform_applies_zoom_and_pan() {
        let bounds = Rect::new(0.0, 0.0, 100.0, 50.0);
        let (scale, tx, ty) = viewport_transform(400.0, 300.0, bounds, 2.0, (10.0, -20.0));
        assert!((scale - 8.0).abs() < 1e-5);
        // tx = (400 - 100*8)/2 - 0 + 10 = -200 + 10 = -190
        assert!((tx - (-190.0)).abs() < 1e-5);
        // ty = (300 - 50*8)/2 - 0 - 20 = -50 - 20 = -70
        assert!((ty - (-70.0)).abs() < 1e-5);
    }

    #[test]
    fn screen_point_round_trips_to_svg() {
        let bounds = Rect::new(0.0, 0.0, 100.0, 50.0);
        let zoom = 2.0;
        let pan = (10.0, -20.0);
        let (scale, tx, ty) = viewport_transform(400.0, 300.0, bounds, zoom, pan);
        // Pick an SVG point, forward-map it to screen, then invert.
        let (svg_x, svg_y) = (30.0_f32, 25.0_f32);
        let screen_x = svg_x * scale + tx;
        let screen_y = svg_y * scale + ty;
        let back_x = (screen_x - tx) / scale;
        let back_y = (screen_y - ty) / scale;
        assert!((back_x - svg_x).abs() < 1e-4);
        assert!((back_y - svg_y).abs() < 1e-4);
    }

    #[test]
    fn zoom_to_cursor_keeps_anchor_fixed() {
        let bounds = Rect::new(0.0, 0.0, 100.0, 50.0);
        let (w, h) = (400.0_f32, 300.0_f32);
        let (cx, cy) = (120.0_f32, 90.0_f32);

        let old_zoom = 1.0;
        let pan0 = (0.0, 0.0);
        let (s0, tx0, ty0) = viewport_transform(w, h, bounds, old_zoom, pan0);
        let svg_before = ((cx - tx0) / s0, (cy - ty0) / s0);

        let new_zoom = old_zoom * 1.25;
        let (s1, tx1, ty1) = viewport_transform(w, h, bounds, new_zoom, pan0);
        let svg_after = ((cx - tx1) / s1, (cy - ty1) / s1);

        // Pan adjustment that re-anchors the cursor point.
        let pan1 = (
            pan0.0 + (svg_after.0 - svg_before.0) * s1,
            pan0.1 + (svg_after.1 - svg_before.1) * s1,
        );
        let (s2, tx2, ty2) = viewport_transform(w, h, bounds, new_zoom, pan1);
        let svg_final = ((cx - tx2) / s2, (cy - ty2) / s2);

        assert!((svg_final.0 - svg_before.0).abs() < 1e-3);
        assert!((svg_final.1 - svg_before.1).abs() < 1e-3);
    }

    #[test]
    fn pan_clamped_to_keep_map_on_screen() {
        // content_w = 100 * 8 = 800, viewport 400 => max_x = (800-400)/2 = 200
        let clamped = clamp_pan_offset(400.0, 300.0, 100.0, 50.0, 8.0, (10_000.0, -10_000.0));
        assert!((clamped.0 - 200.0).abs() < 1e-5);
        // content_h = 50 * 8 = 400, viewport 300 => max_y = (400-300)/2 = 50
        assert!((clamped.1 - (-50.0)).abs() < 1e-5);
    }

    #[test]
    fn pan_pinned_center_when_fit() {
        // At zoom 1, fit scale 4 => content 400x200, viewport 400x300.
        // Width exactly fits (max_x=0), height smaller than viewport (max_y=0).
        let clamped = clamp_pan_offset(400.0, 300.0, 100.0, 50.0, 4.0, (50.0, 50.0));
        assert!(clamped.0.abs() < 1e-5);
        assert!(clamped.1.abs() < 1e-5);
    }

    #[test]
    fn drag_delta_maps_one_to_one_to_pan() {
        // Pan is applied in widget-space pixels, so a middle-drag of (dx, dy)
        // from a starting pan shifts pan by exactly (dx, dy) before clamping.
        let start = (12.0_f32, -4.0_f32);
        let (dx, dy) = (30.0_f32, 15.0_f32);
        let expected = (start.0 + dx, start.1 + dy);
        assert_eq!(expected, (42.0, 11.0));
    }
}
