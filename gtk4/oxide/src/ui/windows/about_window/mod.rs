pub mod imp;


use gtk::{
    gio, 
    glib, 
    prelude::*, 
};


glib::wrapper! {
    pub struct AboutWindow(ObjectSubclass<imp::AboutWindow>)
        @extends gtk::Widget, gtk::Window,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
        gtk::Native, gtk::Root, gtk::ShortcutManager, gio::ActionMap, gio::ActionGroup
    ;
}

#[gtk::template_callbacks]
impl AboutWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder().property("application", application)
            .build()
    }
}