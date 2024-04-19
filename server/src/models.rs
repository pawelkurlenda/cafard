use chrono::{ DateTime, Utc };
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Deserialize, Serialize, Validate, Debug, Clone)]
pub struct NewCache {
    pub value: String,

    #[validate(custom(function = "validate_expiration_datetime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_datetime: Option<DateTime<Utc>>
}

fn validate_expiration_datetime(expiration_datetime: &DateTime<Utc>) -> Result<(), ValidationError> {
    if expiration_datetime > &Utc::now() {
        Ok(())
    } else {
        Err(ValidationError::new("Expiration datetime validation failed"))
    }
}