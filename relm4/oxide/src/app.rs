use log::{
    debug
};
use serde::{Deserialize, Serialize};

use std::rc::Rc;



pub struct App {
    workspace: Workspace,
}


impl App {
    pub fn new() -> Self {
        return Self {
            workspace: Workspace::default()
        };
    }

    pub fn workspace(&self) -> Workspace {
        return self.workspace.clone();
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    path: Option<String>,
    nodes: Vec<RootNode>
}


impl Workspace {
    pub fn new(path: &str, nodes: Vec<RootNode>) -> Self {
        return Self { 
            path: Some(String::from(path)),
            nodes: nodes.clone()
        };
    }

    pub fn nodes(&self) -> Vec<RootNode> {
        return self.nodes.clone();
    }

    pub fn node_push(&mut self, node: &RootNode) {
        self.nodes.push(node.clone());
    }
}

impl Default for Workspace {
    fn default() -> Self {
        return Self {
            path: None,
            nodes: vec![]
        };
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    name: String,
    children: Vec<Node>
}

impl Node {

    pub fn new(name: &str) -> Self {
        return Self {
            name: String::from(name),
            children: vec![]
        };
    }

    pub fn children(&self) -> Vec<Node> {
        return self.children.clone();
    }

    pub fn children_push(&mut self, node: &Node) {
        self.children.push(node.clone());
    }
}

impl Default for Node {
    fn default() -> Self {
        return Self {
            name: String::from(""),
            children: vec![] 
        };
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootNode {
    name: String,
    children: Vec<Node>
}

impl RootNode {

    pub fn new(name: &str) -> Self {
        return Self {
            name: String::from(name),
            children: vec![]
        };
    }

    pub fn children(&self) -> Vec<Node> {
        return self.children.clone();
    }

    pub fn children_push(&mut self, node: &Node) {
        self.children.push(node.clone());
    }
}