use std::collections::HashMap;
use std::sync::Mutex;
use crate::databases::collection::Collection;
use crate::databases::error::DatabaseError;

#[derive(Debug)]
pub struct IndexSchema {
    name: String,
    is_unique: bool,
    fields: Vec<IndexFieldSchema>
}

#[derive(Debug)]
pub struct IndexFieldSchema {
    field_name: String,
    order: IndexFieldOrder
}

#[derive(Debug)]
pub enum IndexFieldOrder {
    ASC,
    DSC
}

impl IndexSchema {
    fn new(fields: Vec<IndexFieldSchema>, is_unique: bool) -> Result<Self, DatabaseError> {
        //fields.iter()
        // todo: check for duplicates, consider to valid that earlier where fields existing is validate
        // combine value for NAME

        Ok(IndexSchema {
            is_unique: is_unique,
            fields: fields,
            name: "".to_string()
        })
    }
}