use gettextrs::gettext;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use libadwaita as adw;
use libadwaita::subclass::prelude::*;
use std::cell::RefCell;

use crate::map_widget::{MapWidget, RegionState};
use crate::quiz::Quiz;
use crate::registry::MapExercise;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/nacho/mundi/ui/map_quiz_view.ui")]
    pub struct MapQuizView {
        #[template_child]
        pub header_title: TemplateChild<adw::WindowTitle>,
        #[template_child]
        pub content_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub prompt_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub attempts_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub stats_label: TemplateChild<gtk::Label>,
        pub map_widget: RefCell<Option<MapWidget>>,
        pub quiz: RefCell<Option<Quiz>>,
        pub exercise: RefCell<Option<MapExercise>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MapQuizView {
        const NAME: &'static str = "MapQuizView";
        type Type = super::MapQuizView;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            MapWidget::ensure_type();
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MapQuizView {}
    impl WidgetImpl for MapQuizView {}
    impl NavigationPageImpl for MapQuizView {}
}

glib::wrapper! {
    pub struct MapQuizView(ObjectSubclass<imp::MapQuizView>)
        @extends adw::NavigationPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl MapQuizView {
    pub fn new(exercise: &MapExercise) -> Self {
        let obj: Self = glib::Object::builder().build();
        obj.setup(exercise);
        obj
    }

    fn setup(&self, exercise: &MapExercise) {
        let imp = self.imp();

        adw::prelude::NavigationPageExt::set_title(self, &exercise.title());
        imp.header_title.set_title(&exercise.title());
        *imp.exercise.borrow_mut() = Some(exercise.clone());

        let map = MapWidget::new();
        map.set_vexpand(true);
        map.set_hexpand(true);
        // Insert map between attempts_label and stats_label
        imp.content_box
            .insert_child_after(&map, Some(&*imp.attempts_label));

        let view = self.downgrade();
        map.connect_closure(
            "region-clicked",
            false,
            glib::closure_local!(move |_widget: &MapWidget, region_id: &str| {
                if let Some(v) = view.upgrade() {
                    v.on_region_clicked(region_id);
                }
            }),
        );

        map.load_svg(exercise.svg_resource);
        *imp.map_widget.borrow_mut() = Some(map);

        let quiz = Quiz::new(exercise.region_ids);
        *imp.quiz.borrow_mut() = Some(quiz);

        self.update_ui();
    }

    fn on_region_clicked(&self, region_id: &str) {
        let imp = self.imp();

        // Scope borrows so they're released before update_ui
        {
            let mut quiz_ref = imp.quiz.borrow_mut();
            let quiz = match quiz_ref.as_mut() {
                Some(q) => q,
                None => return,
            };

            if quiz.is_finished() {
                return;
            }

            let target = quiz.current_region().unwrap().to_string();
            let correct = quiz.answer(region_id);
            let map = imp.map_widget.borrow();
            let map = map.as_ref().unwrap();

            if correct {
                map.set_region_state(&target, RegionState::Correct);
            } else if quiz.attempts_left == 3 {
                // Ran out of attempts (answer() reset to 3 and advanced)
                map.set_region_state(region_id, RegionState::Wrong);
                map.set_region_state(&target, RegionState::Wrong);
            } else {
                // Still has attempts — flash wrong briefly
                map.set_region_state(region_id, RegionState::Wrong);
                let map_weak = map.downgrade();
                let rid = region_id.to_string();
                glib::timeout_add_local_once(std::time::Duration::from_millis(500), move || {
                    if let Some(m) = map_weak.upgrade() {
                        m.set_region_state(&rid, RegionState::Normal);
                    }
                });
            }
        }

        self.update_ui();
    }

    fn update_ui(&self) {
        let imp = self.imp();
        let quiz = imp.quiz.borrow();
        let quiz = match quiz.as_ref() {
            Some(q) => q,
            None => return,
        };

        if quiz.is_finished() {
            imp.prompt_label.set_text(&gettext("Quiz complete!"));
            imp.attempts_label.set_text("");
            self.save_stats(quiz.session_correct, quiz.session_total);
        } else if let Some(region) = quiz.current_region() {
            let translated = gettext(region);
            imp.prompt_label
                .set_text(&format!("{}: {}", gettext("Select"), translated));
            imp.attempts_label.set_text(&format!(
                "{}: {}",
                gettext("Attempts remaining"),
                quiz.attempts_left
            ));
        }

        let exercise = imp.exercise.borrow();
        let (all_correct, all_total) = if let Some(ex) = exercise.as_ref() {
            self.load_alltime_stats(ex)
        } else {
            (0, 0)
        };

        let session_pct = quiz.session_percentage();
        let alltime_pct = if all_total > 0 {
            (all_correct as f64 / all_total as f64) * 100.0
        } else {
            0.0
        };

        imp.stats_label.set_text(&format!(
            "{}: {}/{} ({:.0}%) · {}: {}/{} ({:.0}%)",
            gettext("Session"),
            quiz.session_correct,
            quiz.session_total,
            session_pct,
            gettext("All-time"),
            all_correct,
            all_total,
            alltime_pct,
        ));
    }

    fn load_alltime_stats(&self, exercise: &MapExercise) -> (u32, u32) {
        let settings = gio::Settings::new_full(
            &gio::SettingsSchemaSource::default()
                .unwrap()
                .lookup("io.github.nacho.mundi.stats", true)
                .unwrap(),
            None::<&gio::SettingsBackend>,
            Some(&exercise.stats_path()),
        );
        (settings.uint("correct"), settings.uint("total"))
    }

    fn save_stats(&self, session_correct: u32, session_total: u32) {
        let imp = self.imp();
        let exercise = imp.exercise.borrow();
        if let Some(ex) = exercise.as_ref() {
            let settings = gio::Settings::new_full(
                &gio::SettingsSchemaSource::default()
                    .unwrap()
                    .lookup("io.github.nacho.mundi.stats", true)
                    .unwrap(),
                None::<&gio::SettingsBackend>,
                Some(&ex.stats_path()),
            );
            let prev_correct = settings.uint("correct");
            let prev_total = settings.uint("total");
            let _ = settings.set_uint("correct", prev_correct + session_correct);
            let _ = settings.set_uint("total", prev_total + session_total);
        }
    }
}
