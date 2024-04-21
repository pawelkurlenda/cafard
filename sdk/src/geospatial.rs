use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use geo::{HaversineDistance, Point, point};

/*#[derive(Clone, Debug)]
pub struct GeospatialPoint {
    longitude: f64,
    latitude: f64,
}*/

#[derive(Debug)]
pub struct Geospatial {
    items: Mutex<HashMap<String, Point<f64>>>,
}

impl Geospatial {
    pub fn new() -> Arc<Self> {
        Arc::new(Geospatial {
            items: Mutex::new(HashMap::new()),
        })
    }

    //pub fn upsert_location(&self, key: String, longitude: f64, latitude: f64) {
    pub fn upsert_location(&self, key: String, point: Point) {
        let mut items = self.items.lock().unwrap();

        items.insert(key, point);
    }

    pub fn get_nearby_locations(&self, point: Point, radius_in_meters: i32) {
        let mut items = self.items.lock().unwrap();

        let nearby_locations: Vec<(String, Point<f64>)> = items
            .iter()
            .filter(|(_, point)| point.haversine_distance(point) <= radius_in_meters * 1000.0)
            .map(|(name, point)| (name.clone(), *point))
            .collect();

    }

}