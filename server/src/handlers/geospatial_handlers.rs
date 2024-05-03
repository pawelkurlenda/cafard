use actix_web::{HttpResponse, Responder, web};
use crate::models::request_models::{GetNearbyLocationRequest, LocationRequest};
use crate::models::response_models::{GetNearbyLocationResponse, LocationResponse};
use crate::state::GeospatialState;

pub async fn location_put_handler(params: web::Path<String>, new_location: web::Json<LocationRequest>, geospatial_state: web::Data<GeospatialState>) -> impl Responder {
    let location_key = params.to_string();

    let upsert_result = geospatial_state.locations.upsert_location(location_key, new_location.longitude, new_location.longitude);

    match upsert_result {
        Ok(_) => HttpResponse::Ok().finish(),
        //Err(err) => HttpResponse::BadRequest().json(err)
        Err(err) => HttpResponse::BadRequest().finish()
    }
}

pub async fn location_get_by_id_handler(params: web::Path<String>, geospatial_state: web::Data<GeospatialState>) -> impl Responder {
    let location_key = params.to_string();
    let point = geospatial_state.locations.get_location_by_id(&location_key);

    if point == None {
        HttpResponse::NotFound()
    } else {
        //HttpResponse::Ok().json(point)
        HttpResponse::NotFound()
    }
}

pub async fn location_get_nearby_handler(location_request: web::Json<GetNearbyLocationRequest>, geospatial_state: web::Data<GeospatialState>) -> impl Responder {
    let location_get_nearby_result = geospatial_state.locations.get_nearby_locations(
        location_request.location.longitude,
        location_request.location.latitude,
        location_request.radius);

    match location_get_nearby_result {
        Ok(result) => {
            let result: Vec<GetNearbyLocationResponse> = result.into_iter().map(|x| GetNearbyLocationResponse {
                id: x.0,
                location: LocationResponse {
                    longitude: x.1.x(),
                    latitude: x.1.y()
                }
            }).collect();

            HttpResponse::Ok().json(result)
        },
        Err(error) => HttpResponse::BadRequest().json(error)
    }
}