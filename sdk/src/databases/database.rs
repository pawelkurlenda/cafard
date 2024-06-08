use std::collections::{HashMap, HashSet};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use chrono::Utc;
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

    pub fn add_collection(&mut self, collection: Collection) -> Result<(), DatabaseError> {
        //self.collections.insert(collection.name.clone(), collection);

        Ok(())
    }

    pub fn get_collection(&self, name: &str) -> Option<&Collection> {
        self.collections.get(name)
    }

    pub fn delete_collection(&mut self, name: &str) -> Result<(), DatabaseError> {

        let mut collections_guard = self.collections.get_mut().unwrap();

        if let Some(item) = collections_guard.get(name) {
            collections_guard.remove(name);
        } else {
            return Err(DatabaseError::CollectionDoNotExists)
        }

        Ok(())

        // todo: consider relational database, then do not allow to delete if in relation
    }
}