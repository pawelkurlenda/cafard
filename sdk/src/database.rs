use std::collections::{HashMap, HashSet};
use serde_json::Value;
use std::sync::Arc;

#[derive(Debug, Clone)]
struct Document {
    id: u32,
    data: Value,  // todo: Using serde_json::Value to store arbitrary JSON data
}

#[derive(Debug, Clone)]
struct Collection {
    name: String,
    documents: HashMap<u32, Document>,
    schema: Option<HashMap<String, String>>,
    indexes: Option<HashSet<String>>
}

impl Collection {
    fn new(name: &str) -> Self {
        Collection {
            name: name.to_string(),
            documents: HashMap::new(),
            schema: None,
            indexes: None
        }
    }

    fn insert(&mut self, doc: Document) {
        self.documents.insert(doc.id, doc);
    }

    fn select_by_id(&self, id: u32) -> Option<&Document> {
        self.documents.get(&id)
    }

    fn create_index(&self) -> bool {
        // todo : implement
    }

    fn drop_index_by_name(&self) -> bool {
        // todo : implement
    }
}

#[derive(Debug, Clone)]
pub struct Database {
    collections: HashMap<String, Collection>,
}

impl Database {
    pub fn new() -> Arc<Self> {
        Arc::new(Database {
            collections: HashMap::new(),
        })
    }

    pub fn add_collection(&mut self, collection: Collection) {
        self.collections.insert(collection.name.clone(), collection);
    }

    pub fn get_collection(&self, name: &str) -> Option<&Collection> {
        self.collections.get(name)
    }
}