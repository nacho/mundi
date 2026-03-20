use gio::prelude::*;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use libadwaita as adw;
use libadwaita::subclass::prelude::*;

use crate::window::LearnMapsWindow;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct LearnMapsApplication;

    #[glib::object_subclass]
    impl ObjectSubclass for LearnMapsApplication {
        const NAME: &'static str = "LearnMapsApplication";
        type Type = super::LearnMapsApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for LearnMapsApplication {}
    impl ApplicationImpl for LearnMapsApplication {
        fn startup(&self) {
            self.parent_startup();
            let app = self.obj();
            app.set_resource_base_path(Some("/io/github/nacho/learn-maps"));
            app.setup_actions();
            app.set_accels_for_action("app.quit", &["<Ctrl>Q"]);
            app.set_accels_for_action("window.close", &["<Ctrl>W"]);

            let provider = gtk::CssProvider::new();
            provider.load_from_resource("/io/github/nacho/learn-maps/style.css");
            gtk::style_context_add_provider_for_display(
                &gtk::gdk::Display::default().expect("Could not connect to a display"),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        fn activate(&self) {
            let app = self.obj();
            let window = LearnMapsWindow::new(app.upcast_ref());
            window.load_window_state();
            window.present();
        }
    }
    impl GtkApplicationImpl for LearnMapsApplication {}
    impl AdwApplicationImpl for LearnMapsApplication {}
}

glib::wrapper! {
    pub struct LearnMapsApplication(ObjectSubclass<imp::LearnMapsApplication>)
        @extends adw::Application, gtk::Application, gio::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl LearnMapsApplication {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("application-id", "io.github.nacho.learn-maps")
            .build()
    }

    fn setup_actions(&self) {
        self.add_action_entries(vec![gio::ActionEntry::builder("quit")
            .activate(|app: &Self, _, _| app.quit())
            .build()]);
    }
}
