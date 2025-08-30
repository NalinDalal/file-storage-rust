use std::collections::HashMap;

#[derive(Debug)]
#[warn(dead_code)]
pub struct Object {
    pub id: String,
    pub data: String,
    pub metadata: HashMap<String, String>,
}

pub struct ObjectStorage {
    objects: HashMap<String, Object>,
}

impl ObjectStorage {
    pub fn new() -> Self {
        ObjectStorage {
            objects: HashMap::new(),
        }
    }
    pub fn create(&mut self, id: &str, data: &str, metadata: HashMap<String, String>) {
        let obj = Object {
            id: id.to_string(),
            data: data.to_string(),
            metadata,
        };
        self.objects.insert(id.to_string(), obj);
    }
    pub fn read(&self, id: &str) -> Option<&Object> {
        self.objects.get(id)
    }
    pub fn write(&mut self, id: &str, new_data: &str) {
        if let Some(obj) = self.objects.get_mut(id) {
            obj.data = new_data.to_string();
        }
    }
    pub fn delete(&mut self, id: &str) {
        self.objects.remove(id);
    }
    pub fn list(&self) -> Vec<String> {
        self.objects.keys().cloned().collect()
    }
}

