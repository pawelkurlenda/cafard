use actix_web::web;
use crate::handlers::cache_handlers::{cache_delete_handler, cache_get_handler, cache_keys_get_handler, cache_put_handler};
use crate::handlers::database_handlers::{database_create_collection_handler, database_create_collection_index_handler, database_delete_collection_handler, database_delete_collection_index_handler, database_update_collection_handler, database_update_collection_index_handler};
use crate::handlers::geospatial_handlers::{location_get_by_id_handler, location_put_handler};
use crate::handlers::healthcheck_handlers::health_check_handler;
use crate::handlers::lock_handlers::{lock_acquire_handler, lock_release_handler, lock_status_handler};

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
            .route("/{lock_key}/acquire", web::post().to(lock_acquire_handler))
            .route("/{lock_key}/release", web::post().to(lock_release_handler))
            .route("/{lock_key}", web::get().to(lock_status_handler))
    );
}

pub fn geospatial_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/geospatial")
            .route("/locations/{location_id}", web::get().to(location_get_by_id_handler))
            .route("/locations/{location_id}", web::put().to(location_put_handler))
    );
}

pub fn database_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/database")
            .route("/collections", web::post().to(database_create_collection_handler))
            .route("/collections/{collection_id}", web::put().to(database_update_collection_handler))
            .route("/collections/{collection_id}", web::delete().to(database_delete_collection_handler))
            .route("/collections/{collection_id}/indexes", web::post().to(database_create_collection_index_handler))
            .route("/collections/{collection_id}/indexes/{index_id}", web::put().to(database_update_collection_index_handler))
            .route("/collections/{collection_id}/indexes/{index_id}", web::delete().to(database_delete_collection_index_handler))

    );
}