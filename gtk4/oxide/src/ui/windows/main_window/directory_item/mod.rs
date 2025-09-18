pub struct DirectoryItem {
    name: String,
    item_type: String
}

impl DirectoryItem {

    pub fn new(name: &str, item_type: &str) -> Self {
        Self {
            name: name.to_string(),
            item_type: item_type.to_string()
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn item_type(&self) -> String {
        self.item_type.clone()
    }
}