use log::{
    debug
};

use gtk::{
    gdk,
    glib,
    subclass::prelude::*,
    gio,
    gio::prelude::*
};


#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/org/devphilplus/oxide/preferences.ui")]
pub struct PreferencesWindow {
}

#[glib::object_subclass]
impl ObjectSubclass for PreferencesWindow {
    const NAME: &'static str = "PreferencesWindow";
    type Type = super::PreferencesWindow;
    type ParentType = gtk::Window;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for PreferencesWindow {
    fn constructed(&self) {
        self.parent_constructed();

        // let obj = self.obj();
        // let window = obj.downcast_ref::<gtk::Window>().unwrap();
        // window.set_type_hint(gtk::gdk::WindowTypeHint::Dialog);
    }
}

impl WidgetImpl for PreferencesWindow {}

impl WindowImpl for PreferencesWindow {}