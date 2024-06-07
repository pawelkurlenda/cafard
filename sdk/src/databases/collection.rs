use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
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
    documents: HashMap<u64, Document>,
    file_path: String,
    //schema_1: Option<HashMap<String, String>>,
    schemas: Mutex<Option<CollectionSchema>>,
    //schemas: Mutex<Option<Vec<FieldSchema>>>,
    index_schemas: Mutex<Option<HashMap<String, IndexSchema>>>,
    indexes: Mutex<Option<HashMap<String, u64>>>
}

impl Collection {
    fn new(name: &str) -> Arc<Self> {
        Arc::new(Collection {
            name: name.to_string(),
            file_path: name.to_string(),
            documents: HashMap::new(),
            //schema_1: None,
            //schemas_2: Mutex::new(None),
            schemas: Mutex::new(None),
            index_schemas: Mutex::new(None),
            indexes: Mutex::new(None)
        })
    }

    fn insert_document(&mut self, doc: Document) -> Result<u64, DatabaseError> {
        self.documents.insert(doc.id, doc);

        Ok(0)

        // todo: implement
        //
        // valid documents
        // add to indexes
    }

    fn insert_documents(&mut self, doc: Vec<Document>) -> Result<Vec<u64>, DatabaseError> {
        //self.documents.insert(doc.id, doc);

        Ok(Vec::new())
        // todo: implement
        //
        // valid documents
        // add to indexes
    }

    fn select_document_by_id(&self, id: u64) -> Option<&Document> {
        self.documents.get(&id)
    }

    fn create_index(&self, create_index_request: CreateIndexRequest) -> Result<&str, DatabaseError> {
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

        let mut new_index_schema = IndexSchema::new(create_index_request.fields, create_index_request.is_unique);
        let index_name = &new_index_schema.name;

        index_names_guard.as_mut().unwrap().insert(index_name.to_string(), new_index_schema);

        // todo: document values for index to indexes field

        Ok(index_name)
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

    fn delete_index_by_name(&self, index_name: &str) -> Result<(), DatabaseError> {
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

    fn delete_field(&self, field_name: &str) -> Result<(), DatabaseError> {
        // todo: implement

        let mut schemas_guard = self.schemas.lock().unwrap();
        let mut index_schemas_guard = self.index_schemas.lock().unwrap();
        let mut index_guard = self.indexes.lock().unwrap();
        let mut documents_guard = self.documents.lock().unwrap();

        // remove from
        //
        // schemas,
        // documents,
        // index_schemas, allow to remove if part of index? or return error?
        // indexes

        Ok(())
    }

    fn add_field(&self, field: Vec<FieldSchema>) -> Result<(), DatabaseError> {
        // todo: implement

        // add to
        //
        // schemas,
        // all documents

        Ok(())
    }

    fn update_field(&self, field: Vec<FieldSchema>) -> Result<(), DatabaseError> {
        // todo: implement

        // update in
        //
        // schemas,
        // documents,
        // index_schemas
        // indexes

        Ok(())
    }
}