mod imp;

use gtk::{
    glib,
    subclass::prelude::*,
};



pub struct Entry {
    pub value: String
}


glib::wrapper! {
    pub struct Cell(ObjectSubclass<imp::Cell>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
        gtk::Native, gtk::Orientable
    ;
}


impl Cell {
    pub fn set_value(&self, value: &Entry) {
        self.imp().value.set_text(Some(&value.value));
    }
}


impl Default for Cell {
    fn default() -> Self {
        glib::Object::new()
    }
}