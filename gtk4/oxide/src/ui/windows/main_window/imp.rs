use log::{
    debug
};

use gtk::{
    prelude::*,
    glib,
    glib::{
        clone
    },
    subclass::prelude::*,
    gio,
    gio::prelude::*
};

use crate::ui::windows::about_window::AboutWindow;


#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/org/devphilplus/oxide/main_window.ui")]
pub struct MainWindow {
    #[template_child]
    pub headerbar: TemplateChild<gtk::HeaderBar>,
    #[template_child]
    pub menubar: TemplateChild<gtk::PopoverMenuBar>,
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

        let obj = self.obj().clone();

        let preferences_action = gio::SimpleAction::new("preferences", None);
        preferences_action.connect_activate(move |_, _| {
            debug!("show preferences");

            let app = obj.application().unwrap();

            let about_window = AboutWindow::new(&app);
            about_window.set_transient_for(Some(&obj));
            about_window.present();
        });
        self.obj().add_action(&preferences_action);
    }
}

impl WidgetImpl for MainWindow {}

impl WindowImpl for MainWindow {}

impl ApplicationWindowImpl for MainWindow {}