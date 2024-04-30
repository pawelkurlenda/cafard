use actix_web::{HttpResponse, Responder, web};
use validator::Validate;
use crate::models::CacheRequest;
use crate::state::CacheState;

#[allow(unused_variables)]
pub async fn cache_keys_get_handler(cache_state: web::Data<CacheState>) -> impl Responder {
    let cache_keys = vec!["test_key_1", "test_key_1"];
    HttpResponse::Ok().json(cache_keys)
}

pub async fn cache_get_handler(params: web::Path<String>, cache_state: web::Data<CacheState>) -> impl Responder {
    let cache_key = params.to_string();
    let cache_value = cache_state.cache.get(&cache_key);

    HttpResponse::Ok().json(cache_value)
}

pub async fn cache_put_handler(params: web::Path<String>, new_cache: web::Json<CacheRequest>, cache_state: web::Data<CacheState>) -> impl Responder {
    let cache_key = params.to_string();
    let is_valid = new_cache.validate();
    match is_valid {
        Ok(_) => {
            cache_state.cache.set(cache_key, new_cache.0.value, new_cache.0.expiration_datetime);
            HttpResponse::Ok().finish()
        }
        Err(err) => HttpResponse::BadRequest().json(err)
    }
}

pub async fn cache_delete_handler(params: web::Path<String>, cache_state: web::Data<CacheState>) -> impl Responder {
    let cache_key = params.to_string();
    cache_state.cache.delete(&cache_key);

    HttpResponse::Ok().finish()
}