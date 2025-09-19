
use gtk::{
    glib,
    subclass::prelude::*,
};



#[derive(Clone, Debug)]
pub struct DirectoryItem {
    name: String,
    item_type: String,
    items: Vec<DirectoryItem>
}


impl DirectoryItem {

    pub fn new(name: &str, item_type: &str) -> Self {
        Self {
            name: name.to_string(),
            item_type: item_type.to_string(),
            items: vec![]
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn item_type(&self) -> String {
        self.item_type.clone()
    }

    pub fn items(&self) -> Vec<DirectoryItem> {
        self.items.clone()
    }

    pub fn set_items(&mut self, items: Vec<DirectoryItem>) {
        self.items = items;
    }
}



pub struct Directory {

}