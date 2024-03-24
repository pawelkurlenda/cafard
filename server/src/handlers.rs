use super::state::{AppState, CacheState};
use actix_web::{HttpResponse, web};
use crate::models::NewCache;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let mut visit_count = app_state.visit_count.lock().unwrap();
    *visit_count += 1;

    let response = format!("{} {} times. Redis connection is OK.", app_state.health_check_response, visit_count);
    HttpResponse::Ok().json(response)
}

pub async fn cache_keys_get_handler(cache_state: web::Data<CacheState>) -> HttpResponse {
    let cache_keys = vec!["test_key_1", "test_key_1"];
    HttpResponse::Ok().json(cache_keys)
}

pub async fn cache_get_handler(params: web::Path<(String)>, cache_state: web::Data<CacheState>) -> HttpResponse {
    let cache_key = params.to_string();

    HttpResponse::Ok().json(cache_key)
}

pub async fn cache_put_handler(params: web::Path<(String)>, new_cache: web::Json<NewCache>, cache_state: web::Data<CacheState>) -> HttpResponse {
    let cache_key = params.to_string();

    HttpResponse::Ok().json(new_cache)
}

pub async fn cache_delete_handler(params: web::Path<(String)>, cache_state: web::Data<CacheState>) -> HttpResponse {
    let cache_key = params.to_string();

    HttpResponse::Ok().json("")
}