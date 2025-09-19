mod imp;


use gtk::{
    glib,
    subclass::prelude::*,
};



#[derive(Clone, Debug)]
pub struct Node {
    pub name: String,
    pub children: Vec<Node>
}



impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            children: vec![]
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn children(&self) -> Vec<Node> {
        self.children.clone()
    }

    pub fn set_children(&mut self, children: Vec<Node>) {
        self.children = children;
    }
}


impl Default for Node {
    fn default() -> Self {
        Self {
            name: String::from("unknown"),
            children: vec![]
        }
    }
}



glib::wrapper! {
    pub struct NodeObject(ObjectSubclass<imp::NodeObject>)
    ;
}


impl NodeObject {
    pub fn new(node: Node) -> Self {
        let obj: Self = glib::Object::new::<NodeObject>();
        obj.imp().node.replace(node);
        obj
    }
}