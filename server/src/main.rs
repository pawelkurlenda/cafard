mod state;
mod routes;
mod models;
mod handlers;
mod cache;
mod background_job;

use dotenv::dotenv;
use std::env;
use std::io;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use actix_web::{App, HttpServer, web};
use background_jobs_actix::{ActixTimer, WorkerConfig};
use crate::background_job::{DEFAULT_QUEUE, MyJob, MyState};
use crate::cache::Cache;
use crate::routes::{cache_routes, general_routes};
use crate::state::{AppState, CacheState};

//const DEFAULT_QUEUE: &'static str = "default";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    println!("Hello, world!");

    let shared_data = web::Data::new(AppState {
        health_check_response: "OK".to_string(),
        visit_count: Mutex::new(0),
    });

    // Create an instance of your cache
    let cache: Arc<Cache<String>> = Cache::new();

    use background_jobs_core::memory_storage::Storage;
    let storage = Storage::new(ActixTimer);

    let queue_handle = WorkerConfig::new(storage, move |_| MyState::new("My App"))
        .register::<MyJob>()
        .set_worker_count(DEFAULT_QUEUE, 16)
        .start();

    //queue_handle.queue(MyJob::new(1, 2)).await.unwrap_or_default();
    queue_handle.every(Duration::from_secs(10) , MyJob::new(3, 4)).unwrap_or_default();
    //queue_handle.schedule(MyJob::new(5, 6)).await.unwrap_or_default();

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .app_data(cache.clone())
            .configure(general_routes)
            .configure(cache_routes)
    };

    let hostname_port = env::var("SERVER_HOSTNAME_PORT")
        .expect("SERVER_HOSTNAME_PORT is not set in .env file");

    println!("Starting server at: {}", hostname_port);

    HttpServer::new(app).bind(hostname_port).unwrap().run().await
}
