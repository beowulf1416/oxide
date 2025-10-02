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



impl Node {
    pub fn new(
        name: &str,
        path: &str
    ) -> Self {
        return Self {
            name: String::from(name),
            path: String::from(path),
            parent: None,
            children: vec!()
        };
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn path(&self) -> String {
        return self.path.clone();
    }

    pub fn child_add(&mut self, node: Node) {
        self.children.push(node);
    }
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