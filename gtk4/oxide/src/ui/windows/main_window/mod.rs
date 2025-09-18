pub mod imp;
// pub mod views;
// pub mod item;
pub mod directory_item;
pub mod cell;

use gtk::{
    gio, 
    glib, 
    prelude::*, 
};


glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
        gtk::Native, gtk::Root, gtk::ShortcutManager, gio::ActionMap, gio::ActionGroup
    ;
}

#[gtk::template_callbacks]
impl MainWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder().property("application", application)
            .build()
    }
}