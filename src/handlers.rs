use super::state::AppState;
use actix_web::{web, HttpResponse};
use crate::models::NewCache;

pub fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let mut visit_count = app_state.visit_count.lock().unwrap();
    *visit_count += 1;

    let response = format!("{} {} times. Redis connection is OK.", app_state.health_check_response, visit_count);
    HttpResponse::Ok().json(response)
}

pub fn cache_put_handler(app_state: web::Data<AppState>, params: web::Path<(String,)>, new_cache: web::Json<NewCache>) -> HttpResponse {
    let tuple = params.0;
    let cache_key: String = tuple;

    HttpResponse::Ok().json(response)
}

pub fn cache_get_handler(app_state: web::Data<AppState>, params: web::Path<(String,)>,) -> HttpResponse {
    let tuple = params.0;
    let cache_key: String = tuple;

    HttpResponse::Ok().json(response)
}