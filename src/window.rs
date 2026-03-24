use gettextrs::gettext;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::*;
use libadwaita::subclass::prelude::*;

use std::cell::RefCell;

use crate::map_exercise_view::MapExerciseView;
use crate::registry;

mod imp {
    use super::*;
    use glib::Properties;

    #[derive(Default, gtk::CompositeTemplate, Properties)]
    #[template(resource = "/io/github/nacho/mundi/ui/window.ui")]
    #[properties(wrapper_type = super::MundiWindow)]
    pub struct MundiWindow {
        #[template_child]
        pub navigation_view: TemplateChild<adw::NavigationView>,
        #[template_child]
        pub countries_group: TemplateChild<adw::PreferencesGroup>,
        #[template_child]
        primary_menu_button: TemplateChild<gtk::MenuButton>,
        #[property(get, set = Self::set_main_menu, construct_only, nullable)]
        main_menu: RefCell<Option<gio::MenuModel>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MundiWindow {
        const NAME: &'static str = "MundiWindow";
        type Type = super::MundiWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            MapExerciseView::ensure_type();
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for MundiWindow {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().populate_countries();
        }
    }

    impl MundiWindow {
        fn set_main_menu(&self, menu: Option<&gio::MenuModel>) {
            self.primary_menu_button.set_menu_model(menu);
            *self.main_menu.borrow_mut() = menu.cloned();
        }
    }

    impl WidgetImpl for MundiWindow {}
    impl WindowImpl for MundiWindow {}
    impl ApplicationWindowImpl for MundiWindow {}
    impl AdwApplicationWindowImpl for MundiWindow {}
}

glib::wrapper! {
    pub struct MundiWindow(ObjectSubclass<imp::MundiWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl MundiWindow {
    pub fn new(app: &adw::Application, main_menu: Option<&gio::MenuModel>) -> Self {
        glib::Object::builder()
            .property("application", app)
            .property("main-menu", main_menu)
            .build()
    }

    fn populate_countries(&self) {
        let imp = self.imp();

        for country in registry::countries() {
            let row = adw::ActionRow::builder()
                .title(country.name())
                .activatable(true)
                .build();
            row.add_suffix(&gtk::Image::from_icon_name("go-next-symbolic"));

            let nav = imp.navigation_view.clone();
            let country = country.clone();
            row.connect_activated(move |_| {
                let page = Self::build_country_page(&country);
                nav.push(&page);
            });

            imp.countries_group.add(&row);
        }
    }

    fn build_country_page(country: &registry::Country) -> adw::NavigationPage {
        let prefs_page = adw::PreferencesPage::new();
        let mut current_group: Option<Option<&str>> = None;
        let mut group = adw::PreferencesGroup::new();

        for exercise in country.exercises {
            if current_group != Some(exercise.group) {
                current_group = Some(exercise.group);
                group = adw::PreferencesGroup::new();
                if let Some(g) = exercise.group {
                    group.set_title(&gettext(g));
                }
                prefs_page.add(&group);
            }

            let row = adw::ActionRow::builder()
                .title(exercise.title())
                .activatable(true)
                .build();
            row.add_suffix(&gtk::Image::from_icon_name("go-next-symbolic"));

            let ex = exercise.clone();
            row.connect_activated(move |row| {
                let nav = row
                    .ancestor(adw::NavigationView::static_type())
                    .and_downcast::<adw::NavigationView>()
                    .unwrap();
                let quiz_view = MapExerciseView::new(&ex);
                nav.push(&quiz_view);
            });

            group.add(&row);
        }

        let toolbar = adw::ToolbarView::new();
        toolbar.add_top_bar(&adw::HeaderBar::new());
        toolbar.set_content(Some(&prefs_page));

        adw::NavigationPage::builder()
            .title(country.name())
            .child(&toolbar)
            .build()
    }

    pub fn load_window_state(&self) {
        let settings = gio::Settings::new("io.github.nacho.mundi.state.window");
        let (width, height) = settings.get::<(i32, i32)>("size");
        self.set_default_size(width, height);

        if settings.boolean("maximized") {
            self.maximize();
        }

        self.connect_notify_local(Some("maximized"), move |window, _| {
            let settings = gio::Settings::new("io.github.nacho.mundi.state.window");
            settings
                .set_boolean("maximized", window.is_maximized())
                .unwrap();
        });

        self.connect_notify_local(Some("default-width"), move |window, _| {
            let settings = gio::Settings::new("io.github.nacho.mundi.state.window");
            if !window.is_maximized() {
                settings
                    .set("size", (window.default_width(), window.default_height()))
                    .unwrap();
            }
        });

        self.connect_notify_local(Some("default-height"), move |window, _| {
            let settings = gio::Settings::new("io.github.nacho.mundi.state.window");
            if !window.is_maximized() {
                settings
                    .set("size", (window.default_width(), window.default_height()))
                    .unwrap();
            }
        });
    }
}
