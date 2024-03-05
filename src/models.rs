use std::time::Duration;
use actix_web::cookie::time::{ OffsetDateTime};
use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewCache {
    pub value: String,
    pub expiration_datetime: OffsetDateTime
}
impl From<web::Json<NewCache>> for NewCache {
    fn from(cache: web::Json<NewCache>) -> Self {
        NewCache {
            value: cache.0.value,
            expiration_datetime: cache.0.expiration_datetime,
        }
    }
}