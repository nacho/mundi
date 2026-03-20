use glib::subclass::Signal;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::RefCell;
use std::sync::OnceLock;

#[derive(Clone, Copy, PartialEq)]
pub enum RegionState {
    Normal,
    Highlighted,
    Correct,
    Wrong,
}

pub struct Region {
    pub id: String,
    pub path: gtk::gsk::Path,
    pub bounds: gtk::graphene::Rect,
    pub state: RegionState,
}

mod imp {
    use super::*;

    pub struct MapWidget {
        pub regions: RefCell<Vec<Region>>,
        pub svg_bounds: RefCell<gtk::graphene::Rect>,
    }

    impl Default for MapWidget {
        fn default() -> Self {
            Self {
                regions: RefCell::new(Vec::new()),
                svg_bounds: RefCell::new(gtk::graphene::Rect::new(0.0, 0.0, 1.0, 1.0)),
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
                vec![Signal::builder("region-clicked")
                    .param_types([String::static_type()])
                    .build()]
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

            let svg_bounds = self.svg_bounds.borrow();
            let (sx, sy, sw, sh) = (
                svg_bounds.x(),
                svg_bounds.y(),
                svg_bounds.width(),
                svg_bounds.height(),
            );
            if sw <= 0.0 || sh <= 0.0 {
                return;
            }

            let scale = (width / sw).min(height / sh);
            let tx = (width - sw * scale) / 2.0 - sx * scale;
            let ty = (height - sh * scale) / 2.0 - sy * scale;

            snapshot.save();
            snapshot.translate(&gtk::graphene::Point::new(tx, ty));
            snapshot.scale(scale, scale);

            let stroke = gtk::gsk::Stroke::new(1.5 / scale);
            let border_color = gtk::gdk::RGBA::new(0.3, 0.3, 0.3, 1.0);

            for region in self.regions.borrow().iter() {
                let fill_color = match region.state {
                    RegionState::Normal => gtk::gdk::RGBA::new(0.85, 0.89, 0.93, 1.0),
                    RegionState::Highlighted => gtk::gdk::RGBA::new(0.68, 0.78, 0.9, 1.0),
                    RegionState::Correct => gtk::gdk::RGBA::new(0.3, 0.76, 0.48, 1.0),
                    RegionState::Wrong => gtk::gdk::RGBA::new(0.87, 0.28, 0.28, 1.0),
                };
                snapshot.append_fill(&region.path, gtk::gsk::FillRule::Winding, &fill_color);
                snapshot.append_stroke(&region.path, &stroke, &border_color);
            }

            snapshot.restore();
        }

        fn measure(&self, orientation: gtk::Orientation, _for_size: i32) -> (i32, i32, i32, i32) {
            match orientation {
                gtk::Orientation::Horizontal => (300, 600, -1, -1),
                _ => (200, 500, -1, -1),
            }
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
                    if tag.as_ref() == b"path" {
                        let mut id = None;
                        let mut d = None;
                        for attr in e.attributes().flatten() {
                            match attr.key.as_ref() {
                                b"id" => {
                                    id = Some(String::from_utf8_lossy(&attr.value).to_string())
                                }
                                b"d" => d = Some(String::from_utf8_lossy(&attr.value).to_string()),
                                _ => {}
                            }
                        }
                        if let (Some(id), Some(d)) = (id, d) {
                            let d = d.trim().to_string();
                            if let Ok(path) = gtk::gsk::Path::parse(&d) {
                                let bounds = path
                                    .bounds()
                                    .unwrap_or(gtk::graphene::Rect::new(0.0, 0.0, 0.0, 0.0));
                                regions.push(Region {
                                    id,
                                    path,
                                    bounds,
                                    state: RegionState::Normal,
                                });
                            }
                        }
                    } else if tag.as_ref() == b"circle" {
                        let mut id = None;
                        let mut cx = None;
                        let mut cy = None;
                        for attr in e.attributes().flatten() {
                            match attr.key.as_ref() {
                                b"id" => {
                                    id = Some(String::from_utf8_lossy(&attr.value).to_string())
                                }
                                b"cx" => {
                                    cx = String::from_utf8_lossy(&attr.value).parse::<f32>().ok()
                                }
                                b"cy" => {
                                    cy = String::from_utf8_lossy(&attr.value).parse::<f32>().ok()
                                }
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
        let width = self.width() as f32;
        let height = self.height() as f32;
        let svg_bounds = self.imp().svg_bounds.borrow();
        let (sx, sy, sw, sh) = (
            svg_bounds.x(),
            svg_bounds.y(),
            svg_bounds.width(),
            svg_bounds.height(),
        );
        let scale = (width / sw).min(height / sh);
        let tx = (width - sw * scale) / 2.0 - sx * scale;
        let ty = (height - sh * scale) / 2.0 - sy * scale;
        ((x as f32 - tx) / scale, (y as f32 - ty) / scale)
    }

    fn hit_test(&self, x: f64, y: f64) -> Option<String> {
        let (svg_x, svg_y) = self.transform_to_svg(x, y);
        let point = gtk::graphene::Point::new(svg_x, svg_y);
        let regions = self.imp().regions.borrow();
        for region in regions.iter() {
            if region.path.in_fill(&point, gtk::gsk::FillRule::Winding) {
                return Some(region.id.clone());
            }
        }
        None
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
            let new_state =
                if region.state == RegionState::Correct || region.state == RegionState::Wrong {
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

    pub fn set_region_state(&self, region_id: &str, state: RegionState) {
        let mut regions = self.imp().regions.borrow_mut();
        for region in regions.iter_mut() {
            if region.id == region_id {
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
            region.state = RegionState::Normal;
        }
        drop(regions);
        self.queue_draw();
    }
}
