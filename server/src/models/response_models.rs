use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetNearbyLocationResponse {
    pub id: String,
    pub location: LocationResponse
}

#[derive(Debug, Deserialize)]
pub struct LocationResponse {
    pub longitude: f64,
    pub latitude: f64
}