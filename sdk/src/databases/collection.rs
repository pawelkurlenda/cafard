use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use serde_json::Value;
use crate::databases::error::DatabaseError;

#[derive(Debug, Clone)]
pub struct Document {
    id: u32,
    data: Value,  // todo: Using serde_json::Value to store arbitrary JSON data
}

#[derive(Debug)]
pub struct Collection {
    name: String,
    documents: HashMap<u32, Document>,
    file_path: String,
    schema_1: Option<HashMap<String, String>>,
    schema_2: Option<CollectionSchema>,
    //index_names: Mutex<Option<HashSet<String>>>,
    index_names: Mutex<Option<HashMap<String, IndexSetting>>>,
    indexes: Mutex<Option<HashMap<String, u32>>>
}

#[derive(Debug)]
pub struct CollectionSchema {
    fields: Vec<FieldSchema>
}

#[derive(Debug)]
pub struct FieldSchema {
    name: String,
    is_primary_key: bool,
    is_row_version: bool,
    field_type: String // todo: how to store field type (int, decimal, date, string)
}

#[derive(Debug)]
pub struct IndexSetting {
    name: String,
    is_unique: bool,
    //fields:
}

impl Collection {
    fn new(name: &str) -> Self {
        Collection {
            name: name.to_string(),
            file_path: name.to_string(),
            documents: HashMap::new(),
            schema_1: None,
            schema_2: None,
            index_names: Mutex::new(None),
            indexes: Mutex::new(None)
        }
    }

    fn insert_document(&mut self, doc: Document) {
        self.documents.insert(doc.id, doc);
    }

    fn select_document_by_id(&self, id: u32) -> Option<&Document> {
        self.documents.get(&id)
    }

    fn create_index_1(&self, index_name: &str, is_unique: bool) -> Result<String, DatabaseError> {
        // todo : implement index_name example: name_1_age_1
        Ok("true".to_string())
    }

    fn create_index_2(&self, index_name: HashSet<String>, is_unique: bool) -> Result<String, DatabaseError> {
        // todo : implement
        Ok("true".to_string())
    }

    fn get_index_names(&self) -> Vec<String> {
        let index_names_lock = self.index_names.lock().unwrap();
        if let Some(index_names) = &*index_names_lock {
            index_names.iter().cloned().collect()
        } else {
            Vec::new()
        }
    }

    fn drop_index_by_name(&self, index_name: &str) -> Result<(), DatabaseError> {
        if self.index_names == None {
            return Err(DatabaseError::IndexDoNotExists)
        }

        let mut items = self.index_names.lock().unwrap();
        let a = items.get(index_name);

        if a == None {
            return Err(DatabaseError::IndexDoNotExists)
        }

        items.remove(a);

        // todo: remove from indexes

        let mut items_indexes = self.indexes.lock().unwrap();

        if let Some(indexes) = items_indexes.get(index_name) {
            items_indexes.unwrap().remove(indexes);
        }

        Ok(())
    }

    fn drop_index_by_name_2(&self, index_name: &str) -> Result<(), DatabaseError> {
        let mut index_names_guard = self.index_names.lock().unwrap();
        let mut indexes_guard = self.indexes.lock().unwrap();

        if let Some(index_names) = index_names_guard.as_mut() {
            if index_names.remove(index_name) {
                // If index_name successfully removed from index_names, also try to remove from indexes
                if let Some(indexes) = indexes_guard.as_mut() {
                    indexes.remove(index_name);
                }
                Ok(())
            } else {
                Err(DatabaseError::IndexDoNotExists)
            }
        } else {
            Err(DatabaseError::IndexDoNotExists)
        }
    }
}