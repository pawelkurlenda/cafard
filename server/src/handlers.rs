use super::state::AppState;
use actix_web::{HttpResponse, web};
use crate::models::NewCache;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let mut visit_count = app_state.visit_count.lock().unwrap();
    *visit_count += 1;

    let response = format!("{} {} times. Redis connection is OK.", app_state.health_check_response, visit_count);
    HttpResponse::Ok().json(response)
}

pub async fn cache_get_handler(app_state: web::Data<AppState>, params: web::Path<(String)>) -> HttpResponse {
    let cache_key = params.to_string();

    HttpResponse::Ok().json(cache_key)
}

pub async fn cache_put_handler(app_state: web::Data<AppState>, params: web::Path<(String)>, new_cache: web::Json<NewCache>) -> HttpResponse {
    let cache_key = params.to_string();

    HttpResponse::Ok().json(new_cache)
}
