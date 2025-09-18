use gtk::{
    prelude::*,
    subclass::prelude::*,
    glib
};
use glib_macros::Properties;


#[derive(Default)]
pub struct FileItem {
    pub name: String,
    pub size: String,
}


impl FileItem {

    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn size(&self) -> String {
        return self.size.clone();
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_size(&mut self, size: String) {
        self.size = size;
    }
}

#[glib::object_subclass]
impl ObjectSubclass for FileItem {
    const NAME: &'static str = "FileItem";
    type Type = super::FileItem;
    type ParentType = glib::Object;

    // fn class_init(klass: &mut Self::Class) {
    //     klass.bind_template();
    //     klass.bind_template_instance_callbacks();
    // }

    // fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
    //     obj.init_template();
    // }
}


impl ObjectImpl for FileItem {
    // fn constructed(&self) {
    //     self.parent_constructed();
    // }

    // fn properties() -> &'static [glib::ParamSpec] {
    //     static PROPERTIES: once_cell::sync::Lazy<Vec<glib::ParamSpec>> =
    //         once_cell::sync::Lazy::new(|| {
    //             vec![
    //                 glib::ParamSpecString::builder("name")
    //                     .nick("Name")
    //                     .blurb("The item's name")
    //                     // .mutable(true)
    //                     // .construct_only()
    //                     .readwrite()
    //                     .build(),
    //                 glib::ParamSpecString::builder("size")
    //                     .nick("size")
    //                     .blurb("The item's size")
    //                     // .mutable(true)
    //                     // .construct_only()
    //                     .readwrite()
    //                     .build(),
    //             ]
    //         });
    //     PROPERTIES.as_ref()
    // }

    // fn set_property(
    //     &self,
    //     // _obj: &Self::Type,
    //     _id: usize,
    //     value: &glib::Value,
    //     pspec: &glib::ParamSpec
    // ) {
    //     match pspec.name() {
    //         "name" => {
    //             let _name = value
    //                 .get::<String>()
    //                 .expect("The value is not of type `String`.");
    //         }
    //         "size" => {
    //             let _value = value
    //                 .get::<String>()
    //                 .expect("The size is not of type `String`.");
    //         }
    //         _ => unimplemented!(),
    //     }        
    // }

    // fn property(
    //     &self, 
    //     // _obj: &Self::Type, 
    //     _id: usize, 
    //     pspec: &glib::ParamSpec
    // ) -> glib::Value {
    //     match pspec.name() {
    //         "name" => self.obj().property::<String>("name").to_value(),
    //         "size" => self.obj().property::<String>("size").to_value(),
    //         _ => unimplemented!(),
    //     }
    // }
}