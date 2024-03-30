use actix_web::web;
use chrono::{ DateTime, Utc };
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidateArgs, ValidationError};

#[derive(Deserialize, Serialize, Validate, Debug, Clone)]
pub struct NewCache {
    #[validate(required)]
    pub value: String,

    #[validate(required)]
    #[validate(custom(function = "validate", arg = "(i64, i64)"))]
    pub expiration_datetime: DateTime<Utc>
}

impl From<web::Json<NewCache>> for NewCache {
    fn from(tweet: web::Json<NewCache>) -> Self {
        NewCache {
            value: tweet.0.value,
            expiration_datetime: tweet.0.expiration_datetime,
        }
    }
}

fn validate(expiration_datetime: DateTime<Utc>) -> Result<(), ValidationError> {
    if expiration_datetime > Utc::now() {
        Ok(())
    } else {
        Err(ValidationError::new("Expiration datetime validation failed"))
    }
}

lazy_static! {

}