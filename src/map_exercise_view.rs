use gettextrs::gettext;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use i18n_format::i18n_format;
use libadwaita as adw;
use libadwaita::subclass::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use crate::map_widget::{MapWidget, RegionState};
use crate::quiz::Quiz;
use crate::registry::{ExerciseKind, MapExercise};

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/nacho/mundi/ui/map_exercise_view.ui")]
    pub struct MapExerciseView {
        #[template_child]
        pub header_title: TemplateChild<adw::WindowTitle>,
        #[template_child]
        pub timer_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub quiz_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub content_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub discovery_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub prompt_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub attempts_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub results_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub score_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub score_caption: TemplateChild<gtk::Label>,
        #[template_child]
        pub time_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub time_caption: TemplateChild<gtk::Label>,
        #[template_child]
        pub retry_button: TemplateChild<gtk::Button>,
        pub map_widget: RefCell<Option<MapWidget>>,
        pub quiz: RefCell<Option<Quiz>>,
        pub exercise: RefCell<Option<MapExercise>>,
        pub quiz_active: RefCell<bool>,
        pub start_time: Rc<RefCell<Option<Instant>>>,
        pub timer_source_id: RefCell<Option<glib::SourceId>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MapExerciseView {
        const NAME: &'static str = "MapExerciseView";
        type Type = super::MapExerciseView;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            MapWidget::ensure_type();
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MapExerciseView {}
    impl WidgetImpl for MapExerciseView {}
    impl NavigationPageImpl for MapExerciseView {}
}

glib::wrapper! {
    pub struct MapExerciseView(ObjectSubclass<imp::MapExerciseView>)
        @extends adw::NavigationPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl MapExerciseView {
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

        // Wire quiz button
        let view = self.downgrade();
        imp.quiz_button.connect_clicked(move |_| {
            if let Some(v) = view.upgrade() {
                v.start_quiz();
            }
        });

        // Wire retry button
        let view = self.downgrade();
        imp.retry_button.connect_clicked(move |_| {
            if let Some(v) = view.upgrade() {
                v.start_quiz();
            }
        });
    }

    fn find_region_name(&self, region_id: &str) -> Option<String> {
        let exercise = self.imp().exercise.borrow();
        exercise.as_ref().and_then(|ex| {
            ex.regions
                .iter()
                .find(|(id, _)| *id == region_id)
                .map(|(_, name)| {
                    if ex.kind == ExerciseKind::Capitals {
                        i18n_format!("{}, capital of {}", gettext(region_id), gettext(*name))
                    } else {
                        gettext(*name)
                    }
                })
        })
    }

    fn on_region_clicked(&self, region_id: &str) {
        if *self.imp().quiz_active.borrow() {
            self.on_quiz_click(region_id);
        } else {
            // Discovery mode: show name
            if let Some(name) = self.find_region_name(region_id) {
                self.imp().discovery_label.set_text(&name);
            }
        }
    }

    fn start_quiz(&self) {
        let imp = self.imp();
        let exercise = imp.exercise.borrow().clone();
        let exercise = match exercise {
            Some(ex) => ex,
            None => return,
        };

        // Stop any existing timer
        if let Some(source_id) = imp.timer_source_id.borrow_mut().take() {
            source_id.remove();
        }

        *imp.quiz_active.borrow_mut() = true;
        *imp.quiz.borrow_mut() = Some(Quiz::new(exercise.regions));

        // UI: hide discovery + results, show quiz elements
        imp.quiz_button.set_visible(false);
        imp.discovery_label.set_visible(false);
        imp.results_box.set_visible(false);
        imp.prompt_label.set_visible(true);
        imp.attempts_label.set_visible(true);

        // Show map (in case results were showing)
        if let Some(map) = imp.map_widget.borrow().as_ref() {
            map.set_visible(true);
            map.reset_all_states();
        }

        // Start timer
        *imp.start_time.borrow_mut() = Some(Instant::now());
        imp.timer_label.set_text("0:00");
        imp.timer_label.set_visible(true);

        let start_time = imp.start_time.clone();
        let timer_label = imp.timer_label.clone();
        let source_id = glib::timeout_add_local(std::time::Duration::from_secs(1), move || {
            if let Some(start) = *start_time.borrow() {
                let elapsed = start.elapsed().as_secs();
                timer_label.set_text(&format!("{}:{:02}", elapsed / 60, elapsed % 60));
                glib::ControlFlow::Continue
            } else {
                glib::ControlFlow::Break
            }
        });
        *imp.timer_source_id.borrow_mut() = Some(source_id);

        self.update_quiz_ui();
    }

    fn on_quiz_click(&self, region_id: &str) {
        let imp = self.imp();

        // Ignore clicks on already-resolved regions
        if let Some(map) = imp.map_widget.borrow().as_ref() {
            let state = map.region_state(region_id);
            if state == RegionState::Correct || state == RegionState::Wrong {
                return;
            }
        }

        {
            let mut quiz_ref = imp.quiz.borrow_mut();
            let quiz = match quiz_ref.as_mut() {
                Some(q) => q,
                None => return,
            };

            if quiz.is_finished() {
                return;
            }

            let target = quiz.current_id().unwrap().to_string();
            let correct = quiz.answer(region_id);
            let map = imp.map_widget.borrow();
            let map = map.as_ref().unwrap();

            if correct {
                map.set_region_state(&target, RegionState::Correct);
            } else if quiz.attempts_left == 3 {
                // Ran out of attempts (answer() reset to 3 and advanced)
                // Only the target stays red permanently
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

        self.update_quiz_ui();
    }

    fn update_quiz_ui(&self) {
        let imp = self.imp();
        let quiz = imp.quiz.borrow();
        let quiz = match quiz.as_ref() {
            Some(q) => q,
            None => return,
        };

        if quiz.is_finished() {
            self.show_results(quiz.session_correct, quiz.session_total);
        } else if let Some(name) = quiz.current_name() {
            let exercise = imp.exercise.borrow();
            let translated = if exercise.as_ref().map(|e| e.kind) == Some(ExerciseKind::Capitals) {
                gettext(quiz.current_id().unwrap())
            } else {
                gettext(name)
            };
            imp.prompt_label
                .set_text(&i18n_format!("{}: {}", gettext("Select"), translated));
            imp.attempts_label.set_text(&i18n_format!(
                "{}: {}",
                gettext("Attempts remaining"),
                quiz.attempts_left
            ));
        }
    }

    fn show_results(&self, correct: u32, total: u32) {
        let imp = self.imp();

        // Stop timer
        let elapsed = imp
            .start_time
            .borrow()
            .map(|s| s.elapsed())
            .unwrap_or_default();
        *imp.start_time.borrow_mut() = None;
        if let Some(source_id) = imp.timer_source_id.borrow_mut().take() {
            source_id.remove();
        }
        imp.timer_label.set_visible(false);

        // Save stats
        self.save_stats(correct, total);

        // Hide quiz UI, show results
        imp.prompt_label.set_visible(false);
        imp.attempts_label.set_visible(false);
        if let Some(map) = imp.map_widget.borrow().as_ref() {
            map.set_visible(false);
        }

        let pct = if total > 0 {
            (correct as f64 / total as f64 * 100.0).floor()
        } else {
            0.0
        };
        imp.score_label
            .set_text(&i18n_format!("{}/{}", correct, total));
        imp.score_caption
            .set_text(&i18n_format!("{} — {}%", gettext("Score"), pct));

        let secs = elapsed.as_secs();
        imp.time_label
            .set_text(&format!("{}:{:02}", secs / 60, secs % 60));
        imp.time_caption.set_text(&gettext("Time"));

        imp.results_box.set_visible(true);
        *imp.quiz_active.borrow_mut() = false;
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
