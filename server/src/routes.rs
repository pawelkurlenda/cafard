use actix_web::web;
use crate::handlers::{cache_get_handler, cache_put_handler, health_check_handler};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn cache_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cache")
            .route("/{cache_key}", web::get().to(cache_get_handler))
            .route("/{cache_key}", web::put().to(cache_put_handler))
            .route("/{cache_key}", web::delete().to(cache_delete_handler))
    );
}