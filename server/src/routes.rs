use actix_web::web;
use crate::handlers::{add_location, cache_delete_handler, cache_get_handler, cache_keys_get_handler, cache_put_handler, health_check_handler, lock_acquire, lock_release, lock_status};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn cache_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cache")
            .route("/keys", web::get().to(cache_keys_get_handler))
            .route("/{cache_key}", web::get().to(cache_get_handler))
            .route("/{cache_key}", web::put().to(cache_put_handler))
            .route("/{cache_key}", web::delete().to(cache_delete_handler))
    );
}

pub fn lock_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/lock")
            .route("/{lock_key}/acquire", web::post().to(lock_acquire))
            .route("/{lock_key}/release", web::post().to(lock_release))
            .route("/{lock_key}", web::get().to(lock_status))
    );
}

pub fn geospatial_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/geospatial")
            .route("/locations", web::post().to(add_location))
    );
}