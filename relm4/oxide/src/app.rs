use log::{
    debug,
    error
};
use serde::{Deserialize, Serialize};

use std::rc::Rc;
use std::fs;



pub struct App {
    workspace: Rc<Workspace>,
}


impl App {
    pub fn new() -> Self {
        return Self {
            workspace: Rc::new(Workspace::default())
        };
    }

    pub fn workspace(&self) -> Rc<Workspace> {
        return self.workspace.clone();
    }

    pub fn workspace_mut(&mut self) -> Rc<Workspace> {
        return self.workspace.clone();
    }

    pub fn set_workspace(&mut self, workspace: Workspace) {
        return self.workspace = Rc::new(workspace);
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

    pub fn path(&self) -> Option<String> {
        return self.path.clone();
    }

    // pub fn nodes(&self) -> Vec<RootNode> {
    //     return self.nodes.clone();
    // }

    // pub fn node_push(&mut self, node: &RootNode) {
    //     self.nodes.push(node.clone());
    // }

    pub fn save(&self) -> Result<(), &'static str> {
        debug!("workspace_save: {:?}", self);

        if let Ok(content) = serde_json::to_string(&self) {
            if let Err(e) = fs::write(format!("{}/workspace.json", self.path.clone().unwrap()), content) {
                error!("error occured while writing workspace file: {:?}", e);
            }
            return Ok(());
        } else {
            error!("error occured while writing workspace file");
            return Err("error occured while writing workspace file");
        }
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
    path: String,
    children: Vec<Node>
}

impl Node {

    pub fn new(name: &str, path: &str) -> Self {
        return Self {
            name: String::from(name),
            path: String::from(path),
            children: vec![]
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

    pub fn children_push(&mut self, node: &Node) {
        self.children.push(node.clone());
    }
}

impl Default for Node {
    fn default() -> Self {
        return Self {
            name: String::from(""),
            path: String::from(""),
            children: vec![] 
        };
    }
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootNode {
    name: String,
    path: String,
    #[serde(skip_serializing)]
    children: Vec<Node>
}

impl RootNode {

    pub fn new(name: &str, path: &str) -> Self {
        return Self {
            name: String::from(name),
            path: String::from(path),
            children: vec![]
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

    pub fn children_push(&mut self, node: &Node) {
        self.children.push(node.clone());
    }
}