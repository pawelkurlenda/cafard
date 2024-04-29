use actix_web::{HttpResponse, Responder, web};
use crate::state::AppState;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> impl Responder {
    let mut visit_count = app_state.visit_count.lock().unwrap();
    *visit_count += 1;

    let response = format!("{} {} times. Redis connection is OK.", app_state.health_check_response, visit_count);
    HttpResponse::Ok().json(response)
}

