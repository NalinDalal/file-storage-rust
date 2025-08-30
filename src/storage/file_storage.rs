use std::collections::HashMap;

pub struct FileStorage {
    files: HashMap<String, String>,
}

impl FileStorage {
    pub fn new() -> Self {
        FileStorage {
            files: HashMap::new(),
        }
    }
    pub fn create(&mut self, path: &str, data: &str) {
        self.files.insert(path.to_string(), data.to_string());
    }
    pub fn read(&self, path: &str) -> Option<&String> {
        self.files.get(path)
    }
    pub fn write(&mut self, path: &str, data: &str) {
        if let Some(file) = self.files.get_mut(path) {
            *file = data.to_string();
        }
    }
    pub fn delete(&mut self, path: &str) {
        self.files.remove(path);
    }
    pub fn list(&self) -> Vec<String> {
        self.files.keys().cloned().collect()
    }
}

