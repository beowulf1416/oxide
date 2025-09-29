use log::{
    debug
};

use std::rc::Rc;
use std::boxed::Box;

use serde::{
    Serialize,
    Deserialize
};



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    name: String,
    path: String,

    #[serde(skip_serializing, skip_deserializing)]
    parent: Option<Rc<Node>>,
    #[serde(skip_serializing, skip_deserializing)]
    children: Vec<Node>
}


impl Default for Node {
    fn default() -> Node {
        return Node {
            name: String::from(""),
            path: String::from(""),
            parent: None,
            children: vec!()
        };
    }
}
