mod state;
mod routes;
mod background_job;
mod handlers;
mod models;

use dotenv::dotenv;
use std::env;
//use std::io;
//use std::sync::{Arc, Mutex};
use std::sync::Mutex;
//use std::time::Duration;
use actix_web::{App, HttpServer, web};
//use background_jobs_actix::{ActixTimer, WorkerConfig};
//use crate::background_job::{DEFAULT_QUEUE, MyJob};
use cafard::cache::Cache;
use cafard::database::Database;
use cafard::geospatial::Geospatial;
use cafard::lock::Lock;
use crate::routes::{cache_routes, database_routes, general_routes, geospatial_routes, lock_routes};
use crate::state::{AppState, CacheState, DatabaseState, GeospatialState, LockState};

//const DEFAULT_QUEUE: &'static str = "default";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let shared_data = web::Data::new(AppState {
        health_check_response: "OK".to_string(),
        visit_count: Mutex::new(0),
    });

    let cache_shared_data = web::Data::new(CacheState {
        cache: Cache::new()
    });

    let lock_shared_data = web::Data::new(LockState {
        lock: Lock::new()
    });

    let geospatial_shared_data = web::Data::new(GeospatialState {
        locations: Geospatial::new()
    });

    let database_shared_data = web::Data::new(DatabaseState {
        database_state: Database::new()
    });

    //use background_jobs_core::memory_storage::Storage;
    //let storage = Storage::new(ActixTimer);

    //let queue_handle = WorkerConfig::new(storage, move |_| cache_shared_data.clone())
    //    .register::<MyJob>()
    //    .set_worker_count(DEFAULT_QUEUE, 16)
    //    .start();

    ////queue_handle.queue(MyJob::new(1, 2)).await.unwrap_or_default();
    //queue_handle.every(Duration::from_secs(10) , MyJob::new()).unwrap_or_default();
    ////queue_handle.schedule(MyJob::new(5, 6)).await.unwrap_or_default();

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .app_data(cache_shared_data.clone())
            .app_data(lock_shared_data.clone())
            .app_data(geospatial_shared_data.clone())
            .app_data(database_shared_data.clone())
            .configure(general_routes)
            .configure(cache_routes)
            .configure(lock_routes)
            .configure(geospatial_routes)
            .configure(database_routes)
    };

    let hostname_port = env::var("SERVER_HOSTNAME_PORT")
        .expect("SERVER_HOSTNAME_PORT is not set in .env file");

    HttpServer::new(app).bind(hostname_port).unwrap().run().await
}
