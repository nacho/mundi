use gtk::prelude::*;
use gtk::subclass::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::*;
use libadwaita::subclass::prelude::*;

use crate::map_quiz_view::MapQuizView;
use crate::registry;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/nacho/learn-maps/ui/window.ui")]
    pub struct LearnMapsWindow {
        #[template_child]
        pub navigation_view: TemplateChild<adw::NavigationView>,
        #[template_child]
        pub exercises_group: TemplateChild<adw::PreferencesGroup>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LearnMapsWindow {
        const NAME: &'static str = "LearnMapsWindow";
        type Type = super::LearnMapsWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            MapQuizView::ensure_type();
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for LearnMapsWindow {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().populate_exercises();
        }
    }
    impl WidgetImpl for LearnMapsWindow {}
    impl WindowImpl for LearnMapsWindow {}
    impl ApplicationWindowImpl for LearnMapsWindow {}
    impl AdwApplicationWindowImpl for LearnMapsWindow {}
}

glib::wrapper! {
    pub struct LearnMapsWindow(ObjectSubclass<imp::LearnMapsWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl LearnMapsWindow {
    pub fn new(app: &adw::Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    fn populate_exercises(&self) {
        let imp = self.imp();
        let mut current_country = String::new();

        for exercise in registry::exercises() {
            let country = exercise.country_name();
            if country != current_country {
                if !current_country.is_empty() {
                    // Add a visual separator between countries by using a new group
                }
                current_country = country.clone();
            }

            let row = adw::ActionRow::builder()
                .title(exercise.title())
                .subtitle(&country)
                .activatable(true)
                .build();
            row.add_suffix(&gtk::Image::from_icon_name("go-next-symbolic"));

            let nav = imp.navigation_view.clone();
            let ex = exercise.clone();
            row.connect_activated(move |_| {
                let quiz_view = MapQuizView::new(&ex);
                nav.push(&quiz_view);
            });

            imp.exercises_group.add(&row);
        }
    }

    pub fn load_window_state(&self) {
        let settings = gio::Settings::new("io.github.nacho.learn-maps.state.window");
        let (width, height) = settings.get::<(i32, i32)>("size");
        self.set_default_size(width, height);

        if settings.boolean("maximized") {
            self.maximize();
        }

        self.connect_notify_local(Some("maximized"), move |window, _| {
            let settings = gio::Settings::new("io.github.nacho.learn-maps.state.window");
            settings
                .set_boolean("maximized", window.is_maximized())
                .unwrap();
        });

        self.connect_notify_local(Some("default-width"), move |window, _| {
            let settings = gio::Settings::new("io.github.nacho.learn-maps.state.window");
            if !window.is_maximized() {
                settings
                    .set("size", (window.default_width(), window.default_height()))
                    .unwrap();
            }
        });

        self.connect_notify_local(Some("default-height"), move |window, _| {
            let settings = gio::Settings::new("io.github.nacho.learn-maps.state.window");
            if !window.is_maximized() {
                settings
                    .set("size", (window.default_width(), window.default_height()))
                    .unwrap();
            }
        });
    }
}
