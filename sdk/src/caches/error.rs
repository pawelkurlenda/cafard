use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum CacheError {
}