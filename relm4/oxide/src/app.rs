use log::{
    debug
};
use serde::{Deserialize, Serialize};



pub struct App {
    workspace: Workspace,
}


impl App {
    pub fn new() -> Self {
        return Self {
            workspace: Workspace::new()
        };
    }

    pub fn workspace(&self) -> Workspace {
        return self.workspace.clone();
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    path: Option<String>
}


impl Workspace {
    pub fn new() -> Self {
        return Self { 
            path: None
        };
    }

    pub fn set_path(&self, path: String) -> Self {
        return Self {
            path: Some(path)
        };
    }
}