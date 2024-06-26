use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum GeospatialError {
    #[error("Error")]
    Error = 1,
    #[error("Latitude must be between -90.0 and 90.0")]
    LatitudeOutOfRange = 2,
    #[error("Longitude must be between -180.0 and 180.0")]
    LongitudeOutOfRange = 3
}