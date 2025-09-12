pub mod imp;


use gtk::{
    gio, 
    glib
};



glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for Application {
    fn default() -> Self {
        glib::Object::builder()
            .property("application-id", "org.devphilplus.oxide")
            .build()
    }
}