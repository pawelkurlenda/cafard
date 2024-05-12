use std::collections::{HashMap, HashSet};
use std::io::Error;
use serde_json::Value;
use std::sync::{Arc, Mutex};
use crate::databases::error::DatabaseError;

#[derive(Debug, Clone)]
struct Document {
    id: u32,
    data: Value,  // todo: Using serde_json::Value to store arbitrary JSON data
}

#[derive(Debug, Clone)]
struct Collection {
    name: String,
    documents: HashMap<u32, Document>,
    file_path: String,
    schema: Option<HashMap<String, String>>,
    indexes: Option<HashSet<String>>
}

impl Collection {
    fn new(name: &str) -> Self {
        Collection {
            name: name.to_string(),
            file_path: name.to_string(),
            documents: HashMap::new(),
            schema: None,
            indexes: None
        }
    }

    fn insert_document(&mut self, doc: Document) {
        self.documents.insert(doc.id, doc);
    }

    fn select_document_by_id(&self, id: u32) -> Option<&Document> {
        self.documents.get(&id)
    }

    fn create_index_1(&self, index_name: &str, is_unique: bool) -> Result<String, DatabaseError> {
        // todo : implement
        Ok("true".to_string())
    }

    fn create_index_2(&self, index_name: HashSet<String>, is_unique: bool) -> Result<String, DatabaseError> {
        // todo : implement
        Ok("true".to_string())
    }

    fn drop_index_by_name(&self, index_name: &str) -> Result<(), DatabaseError> {
        // todo : implement
        Ok(())
    }
}

#[derive(Debug)]
pub struct Database {
    collections: Mutex<HashMap<String, Collection>>,
}

impl Database {
    pub fn new() -> Arc<Self> {
        Arc::new(Database {
            collections: Mutex::new(HashMap::new()),
        })
    }

    pub fn add_collection(&mut self, collection: Collection) {
        self.collections.insert(collection.name.clone(), collection);
    }

    pub fn get_collection(&self, name: &str) -> Option<&Collection> {
        self.collections.get(name)
    }
}