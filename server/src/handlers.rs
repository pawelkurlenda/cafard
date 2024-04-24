use super::state::{AppState, CacheState, GeospatialState, LockState};
use actix_web::{HttpResponse, Responder, web};
use validator::Validate;
use crate::models::{NewCache, NewLocation};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> impl Responder {
    let mut visit_count = app_state.visit_count.lock().unwrap();
    *visit_count += 1;

    let response = format!("{} {} times. Redis connection is OK.", app_state.health_check_response, visit_count);
    HttpResponse::Ok().json(response)
}

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

pub async fn cache_put_handler(params: web::Path<String>, new_cache: web::Json<NewCache>, cache_state: web::Data<CacheState>) -> impl Responder {
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

pub async fn lock_acquire_handler(params: web::Path<String>, lock_state: web::Data<LockState>) -> impl Responder {
    let lock_key = params.to_string();
    let is_acquired = lock_state.lock.try_acquire(&lock_key);

    HttpResponse::Ok().json(is_acquired)
}

pub async fn lock_release_handler(params: web::Path<String>, lock_state: web::Data<LockState>) -> impl Responder {
    let lock_key = params.to_string();
    let is_released = lock_state.lock.try_release(&lock_key);

    HttpResponse::Ok().json(is_released)
}

pub async fn lock_status_handler(params: web::Path<String>, lock_state: web::Data<LockState>) -> impl Responder {
    let lock_key = params.to_string();
    let is_acquire = lock_state.lock.is_acquire(&lock_key);

    HttpResponse::Ok().json(is_acquire)
}

pub async fn location_put_handler(params: web::Path<String>, new_location: web::Json<NewLocation>, geospatial_state: web::Data<GeospatialState>) -> impl Responder {
    let location_key = params.to_string();

    geospatial_state.locations.upsert_location(location_key, new_location.longitude, new_location.longitude);

    HttpResponse::Ok().finish()
}

pub async fn location_get_by_id_handler(params: web::Path<String>, geospatial_state: web::Data<GeospatialState>) -> impl Responder {
    let location_key = params.to_string();
    let point = geospatial_state.locations.get_location_by_id(&location_key);

    if point == None {
        HttpResponse::NotFound()
    } else {
        HttpResponse::Ok().json(point)
    }
}

pub async fn location_get_nearby_handler() -> impl Responder {
    HttpResponse::Ok().finish()
}