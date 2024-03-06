use std::time::Duration;
use actix_web::cookie::time::{ OffsetDateTime};
use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewCache {
    pub value: String,
    pub expiration_datetime: chrono::DateTime<chrono::Utc>
}
impl From<web::Json<NewCache>> for NewCache {
    fn from(tweet: web::Json<NewCache>) -> Self {
        NewCache {
            value: tweet.0.value,
            expiration_datetime: tweet.0.expiration_datetime,
        }
    }
}