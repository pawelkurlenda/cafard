use actix_web::{HttpResponse, Responder, web};
use crate::models::NewLocation;
use crate::state::GeospatialState;

pub async fn location_put_handler(params: web::Path<String>, new_location: web::Json<NewLocation>, geospatial_state: web::Data<GeospatialState>) -> impl Responder {
    let location_key = params.to_string();

    let upsert_result = geospatial_state.locations.upsert_location(location_key, new_location.longitude, new_location.longitude);

    match upsert_result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::BadRequest().json(err)
    }
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