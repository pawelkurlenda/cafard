use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewCache {
    pub value: String,
}
impl From<web::Json<NewCache>> for NewCache {
    fn from(cache: web::Json<NewCache>) -> Self {
        NewCache {
            value: cache.value,
        }
    }
}