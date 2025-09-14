use log::{
    debug
};

use gtk::{
    prelude::*,
    glib,
    glib::clone,
    subclass::prelude::*,
    gio,
    gio::prelude::*
};


#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/org/devphilplus/oxide/main_window.ui")]
pub struct MainWindow {
    #[template_child]
    pub headerbar: TemplateChild<gtk::HeaderBar>,
    #[template_child]
    pub menubar: TemplateChild<gtk::PopoverMenuBar>,
}


impl MainWindow {

    fn add_actions(&self) {
        let obj = self.obj().clone();

        let preferences_action = crate::ui::actions::preferences::preferences_action(&obj);
        self.obj().add_action(&preferences_action);

        let about_action = crate::ui::actions::about::about_action(&obj);
        self.obj().add_action(&about_action);

        let file_open_action = crate::ui::actions::file_open::file_open_action(&obj);
        self.obj().add_action(&file_open_action);
    }
}

#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    const NAME: &'static str = "MainWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MainWindow {
    fn constructed(&self) {
        self.parent_constructed();

        self.add_actions();
    }
}

impl WidgetImpl for MainWindow {}

impl WindowImpl for MainWindow {}

impl ApplicationWindowImpl for MainWindow {}