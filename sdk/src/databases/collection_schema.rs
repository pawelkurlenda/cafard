use std::collections::HashMap;
use crate::databases::error::DatabaseError;
use crate::databases::field::FieldSchema;

#[derive(Debug)]
pub struct CollectionSchema {
    fields: Vec<FieldSchema>
}

impl CollectionSchema {
    fn check_fields_availability(&self, fields_to_check: Vec<&str>) -> Result<(), DatabaseError> {
        let mut counts = HashMap::new();
        for field_to_check in fields_to_check {
            let count = counts.entry(field_to_check).or_insert(0);
            *count += 1;
            if *count > 1 {
                return Err(DatabaseError::IndexFieldDuplication)
            }

            let is_field_exists = &self.fields.iter().any(|&x| x.name == field_to_check);

            if !is_field_exists {
                return Err(DatabaseError::FieldDoesNotExists)
            }
        }

        Ok(())
    }
}