use gtk::{
    prelude::*,
    subclass::prelude::*,
    glib
};



#[derive(glib::Properties, Default, Clone)]
#[properties(wrapper_type = glib::Object)]
pub struct FileItem {
    #[property(get, set)]
    pub name: String,
    #[property(get, set)]
    pub size: String,
}