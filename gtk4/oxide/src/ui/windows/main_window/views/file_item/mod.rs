pub mod imp;


use gtk::{
    prelude::*,
    glib
};


glib::wrapper! {
    pub struct FileItem(ObjectSubclass<imp::FileItem>)
        // @extends glib::Object
    ;
}


impl FileItem {
    pub fn new(name: String, size: String) -> Self {
        // glib::Object::new::<Self>(&[("name", &name), ("size", &size)]).expect("Failed to create FileItem")
        // glib::Object::builder()
        //     .property("name", name)
        //     .property("size", size)
        //     .build()

        return Self::new(
            name,
            size
        );
    }

    // pub fn name(&self) -> String {
    //     self.property::<String>("name")
    // }
}