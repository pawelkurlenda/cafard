use std::collections::{HashMap, HashSet};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex, RwLock};
use crate::geospatial::Geospatial;

/// Represents a single document within the database.
#[derive(Debug, Clone)]
struct Document {
    id: u32,
    data: Value,  // Using serde_json::Value to store arbitrary JSON data
}

/// A collection of documents.
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

    /// Insert a new document into the collection.
    fn insert(&mut self, doc: Document) {
        self.documents.insert(doc.id, doc);
    }

    /// Simple SELECT-like operation, searching by ID.
    fn select_by_id(&self, id: u32) -> Option<&Document> {
        self.documents.get(&id)
    }
}

/// The main database structure.
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

    /// Add a new collection to the database.
    pub fn add_collection(&mut self, collection: Collection) {
        self.collections.insert(collection.name.clone(), collection);
    }

    /// Get a collection by name.
    pub fn get_collection(&self, name: &str) -> Option<&Collection> {
        self.collections.get(name)
    }
}