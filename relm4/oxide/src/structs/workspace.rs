use serde::{
    Serialize,
    Deserialize
};

use crate::structs::node::Node;



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    name: String,
    path: String,

    #[serde[skip_serializing, skip_deserializing]]
    children: Vec<Node>
}


impl Workspace {

    pub fn new(
        name: &str,
        path: &str,
    ) -> Self {
        return Self {
            name: String::from(name),
            path: String::from(path),
            children: vec!()
        };
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn path(&self) -> String {
        return self.path.clone();
    }

    pub fn children(&self) -> Vec<Node> {
        return self.children.clone();
    }

    pub fn child_add(&mut self, node: &Node) {
        self.children.push(node.clone());
    }
}


impl Default for Workspace {
    fn default() -> Workspace {
        return Workspace {
            name: String::from(""),
            path: String::from(""),
            children: vec!()
        };
    }
}