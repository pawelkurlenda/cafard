mod state;
mod routes;
mod models;
mod handlers;
mod cache;
mod background_job;

use dotenv::dotenv;
use std::env;
use std::io;
use std::sync::Mutex;
use actix_web::{App, HttpServer, web};
use crate::routes::{cache_routes, general_routes};
use crate::state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    println!("Hello, world!");

    let shared_data = web::Data::new(AppState {
       health_check_response: "OK".to_string(),
       visit_count: Mutex::new(0),
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(cache_routes)
    };

    let hostname_port = env::var("SERVER_HOSTNAME_PORT")
        .expect("SERVER_HOSTNAME_PORT is not set in .env file");

    println!("Starting server at: {}", hostname_port);

    HttpServer::new(app).bind(hostname_port).unwrap().run().await
}
