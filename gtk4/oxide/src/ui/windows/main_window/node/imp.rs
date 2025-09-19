use std::cell::RefCell;

use gtk::{
    glib,
    prelude::*,
    subclass::prelude::*,
};

use once_cell::sync::Lazy;


use super::Node;


#[derive(Default)]
pub struct NodeObject {
    pub node: RefCell<Node>
}


#[glib::object_subclass]
impl ObjectSubclass for NodeObject {
    const NAME: &'static str = "NodeObject";
    type Type = super::NodeObject;
    type ParentType = glib::Object;
}


impl ObjectImpl for NodeObject {
    // fn properties() -> &'static [glib::ParamSpec] {
    //     static PROPS: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
    //         vec![glib::ParamSpec::string(
    //             "name",
    //             "Name",
    //             "Name of the node",
    //             None,
    //             glib::ParamFlags::READABLE,
    //         )]
    //     });
    //     PROPS.as_ref()
    // }

    // fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
    //     match pspec.name() {
    //         "name" => self.node.borrow().name.clone().to_value(),
    //         _ => unimplemented!(),
    //     }
    // }
}