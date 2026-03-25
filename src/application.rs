use gio::prelude::*;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::*;
use libadwaita::subclass::prelude::*;

use crate::window::MundiWindow;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct MundiApplication;

    #[glib::object_subclass]
    impl ObjectSubclass for MundiApplication {
        const NAME: &'static str = "MundiApplication";
        type Type = super::MundiApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for MundiApplication {}
    impl ApplicationImpl for MundiApplication {
        fn startup(&self) {
            self.parent_startup();
            let app = self.obj();
            app.setup_actions();
            app.set_accels_for_action("app.quit", &["<Ctrl>Q"]);
            app.set_accels_for_action("window.close", &["<Ctrl>W"]);

            let provider = gtk::CssProvider::new();
            provider.load_from_resource("/io/github/nacho/mundi/style.css");
            gtk::style_context_add_provider_for_display(
                &gtk::gdk::Display::default().expect("Could not connect to a display"),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        fn activate(&self) {
            let app = self.obj();
            let menu = app.menu_by_id("main-menu");
            let window = MundiWindow::new(app.upcast_ref(), menu.as_ref().map(|m| m.upcast_ref()));
            window.load_window_state();
            window.present();
        }
    }
    impl GtkApplicationImpl for MundiApplication {}
    impl AdwApplicationImpl for MundiApplication {}
}

glib::wrapper! {
    pub struct MundiApplication(ObjectSubclass<imp::MundiApplication>)
        @extends adw::Application, gtk::Application, gio::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl MundiApplication {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("application-id", "io.github.nacho.mundi")
            .property("resource-base-path", "/io/github/nacho/mundi")
            .build()
    }

    fn setup_actions(&self) {
        self.add_action_entries(vec![
            gio::ActionEntry::builder("quit")
                .activate(|app: &Self, _, _| app.quit())
                .build(),
            gio::ActionEntry::builder("about")
                .activate(|app: &Self, _, _| app.show_about())
                .build(),
        ]);
    }

    fn show_about(&self) {
        let dialog = adw::AboutDialog::builder()
            .application_name("Mundi")
            .application_icon("io.github.nacho.mundi")
            .version(crate::config::VERSION)
            .developer_name("Ignacio Casal Quinteiro")
            .license_type(gtk::License::Gpl30)
            .website("https://github.com/nacho/mundi")
            .issue_url("https://github.com/nacho/mundi/issues")
            .build();
        let window = self.active_window();
        dialog.present(window.as_ref());
    }
}
