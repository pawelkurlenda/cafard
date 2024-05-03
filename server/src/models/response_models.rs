use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetNearbyLocationResponse {
    pub id: String,
    pub location: LocationResponse
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationResponse {
    pub longitude: f64,
    pub latitude: f64
}