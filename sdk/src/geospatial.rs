use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct GeospatialItem {
    longitude: f32,
    latitude: f32,
}

#[derive(Debug)]
pub struct Geospatial {
    items: Mutex<HashMap<String, GeospatialItem>>,
}

impl Geospatial {
    pub fn new() -> Arc<Self> {
        Arc::new(Geospatial {
            items: Mutex::new(HashMap::new()),
        })
    }

}