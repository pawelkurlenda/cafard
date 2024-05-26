use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use serde_json::Value;
use crate::databases::collection_schema::CollectionSchema;
use crate::databases::document::Document;
use crate::databases::error::DatabaseError;
use crate::databases::field::FieldSchema;
use crate::databases::index::IndexSchema;
use crate::databases::request_models::index::CreateIndexRequest;

#[derive(Debug)]
pub struct Collection {
    name: String,
    documents: HashMap<u32, Document>,
    file_path: String,
    //schema_1: Option<HashMap<String, String>>,
    schemas: Mutex<Option<CollectionSchema>>,
    //schemas: Mutex<Option<Vec<FieldSchema>>>,
    index_schemas: Mutex<Option<HashMap<String, IndexSchema>>>,
    indexes: Mutex<Option<HashMap<String, u32>>>
}

impl Collection {
    fn new(name: &str) -> Self {
        Collection {
            name: name.to_string(),
            file_path: name.to_string(),
            documents: HashMap::new(),
            //schema_1: None,
            //schemas_2: Mutex::new(None),
            schemas: Mutex::new(None),
            index_schemas: Mutex::new(None),
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

    fn create_index(&self, create_index_request: CreateIndexRequest) -> Result<String, DatabaseError> {
        let mut index_names_guard = self.index_schemas.lock().unwrap();
        let schemas_guard = self.schemas.lock().unwrap();
        let mut indexes_guard = self.indexes.lock().unwrap();

        let field_names = create_index_request.fields
            .iter()
            .map(|item| item.field_name)
            .collect();

        let check_fields_availability_result = schemas_guard.unwrap().check_fields_availability(field_names);

        if check_fields_availability_result.is_err() {
            return Err(check_fields_availability_result.err().unwrap());
        }

        let new_index_schema = IndexSchema::new(create_index_request.fields, create_index_request.is_unique);
        let index_name = new_index_schema.name;

        index_names_guard.as_mut().unwrap().insert(index_name, new_index_schema);

        // todo: document values for index to indexes field

        Ok(new_index_schema.name.to_string())
    }

    fn get_index_schema_by_name(&self, index_name: &str) -> Result<&IndexSchema, DatabaseError> {
        let mut index_names_guard = self.index_schemas.lock().unwrap();

        if let Some(index_namee) = index_names_guard.as_mut().unwrap().get(index_name) {
            Ok(index_namee)
        } else {
            Err(DatabaseError::IndexDoNotExists)
        }
    }

    fn get_index_schemas(&self) -> Vec<&IndexSchema> {
        let index_names_guard = self.index_schemas.lock().unwrap();
        if let Some(ref indices) = *index_names_guard {
            indices.values().cloned().collect()
        } else {
            Vec::new()
        }
    }

    fn get_index_names(&self) -> Vec<String> {
        let index_names_lock = self.index_schemas.lock().unwrap();
        if let Some(index_names) = &*index_names_lock {
            index_names.iter().cloned().collect()
        } else {
            Vec::new()
        }
    }

    fn drop_index_by_name(&self, index_name: &str) -> Result<(), DatabaseError> {
        let mut index_names_guard = self.index_schemas.lock().unwrap();
        let mut indexes_guard = self.indexes.lock().unwrap();

        if let Some(index_names) = index_names_guard.as_mut() {
            if index_names.remove(index_name) {
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