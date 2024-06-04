use std::collections::HashMap;
use crate::databases::error::DatabaseError;
use crate::databases::field::FieldSchema;

#[derive(Debug)]
pub struct CollectionSchema {
    fields: Vec<FieldSchema>
}

impl CollectionSchema {
    pub fn check_fields_availability(&self, fields_to_check: Vec<&str>) -> Result<(), DatabaseError> {
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

    fn try_remove_field(&self, field_name: &str) -> Result<(), DatabaseError> {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> CollectionSchema {
        CollectionSchema {
            fields: vec![
                FieldSchema {
                    name: "id".to_string(),
                    is_primary_key: true,
                    is_autoincrement: true,
                    is_row_version: false,
                    field_type: "int".to_string(),
                },
                FieldSchema {
                    name: "username".to_string(),
                    is_primary_key: false,
                    is_autoincrement: false,
                    is_row_version: false,
                    field_type: "varchar".to_string(),
                },
                FieldSchema {
                    name: "email".to_string(),
                    is_primary_key: false,
                    is_autoincrement: false,
                    is_row_version: false,
                    field_type: "varchar".to_string(),
                },
            ],
        }
    }

    #[test]
    fn test_successful_field_check() {
        let schema = setup();
        let fields_to_check = vec!["id", "username"];
        assert!(schema.check_fields_availability(fields_to_check).is_ok());
    }

    #[test]
    fn test_field_does_not_exist() {
        let schema = setup();
        let fields_to_check = vec!["id", "nonexistent"];
        match schema.check_fields_availability(fields_to_check) {
            Err(DatabaseError::FieldDoesNotExists) => assert!(true),
            _ => assert!(false, "Field does not exist error not triggered."),
        }
    }

    #[test]
    fn test_duplicate_fields() {
        let schema = setup();
        let fields_to_check = vec!["id", "id"];
        match schema.check_fields_availability(fields_to_check) {
            Err(DatabaseError::IndexFieldDuplication) => assert!(true),
            _ => assert!(false, "Duplicate fields error not triggered."),
        }
    }
}