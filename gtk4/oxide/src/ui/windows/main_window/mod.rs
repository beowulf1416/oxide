pub mod imp;
pub mod directory_item;
pub mod node;
pub mod cell;


use log::{
    debug,
    error
};

use gtk::{
    gio, 
    glib, 
    prelude::*, 
    subclass::prelude::*,
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

    pub fn open_workspace(&self, path: &str) {
        debug!("Opening workspace at path: {}", path);
        let imp = self.imp();
        imp.load_workspace(&path);
    }
}