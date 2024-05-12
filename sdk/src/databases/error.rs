use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum DatabaseError {
    #[error("Error")]
    Error = 1,
    #[error("Database do not exists")]
    DatabaseDoNotExists = 2,
    #[error("Collection do not exists")]
    CollectionDoNotExists = 3,
    #[error("Index do not exists")]
    IndexDoNotExists = 4,
    #[error("Index already exists")]
    IndexAlreadyExists = 5,
    #[error("Unique index duplication")]
    UniqueIndexDuplication = 6,
}