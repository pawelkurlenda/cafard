use std::collections::{HashMap, HashSet};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use crate::databases::collection::Collection;
use crate::databases::error::DatabaseError;

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
        //self.collections.insert(collection.name.clone(), collection);
    }

    pub fn get_collection(&self, name: &str) -> Option<&Collection> {
        self.collections.get(name)
    }
}