use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LocationResponse {
    pub longitude: f64,
    pub latitude: f64
}