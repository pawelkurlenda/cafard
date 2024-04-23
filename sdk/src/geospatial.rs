use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use geo::{HaversineDistance, Point};

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

    pub fn upsert_location(&self, key: String, point: Point<f64>) {
        let mut items = self.items.lock().unwrap();

        items.insert(key, point);
    }

    pub fn get_nearby_locations(&self, central_point: &Point, radius_in_meters: i32) -> Vec<(String, Point<f64>)> {
        let mut items = self.items.lock().unwrap();

        let radius = radius_in_meters as f64 * 1000.0;

        let nearby_locations: Vec<(String, Point<f64>)> = items
            .iter()
            .filter(|(_, point)| central_point.haversine_distance(point) <= radius)
            .map(|(name, point)| (name.clone(), *point))
            .collect();

        nearby_locations
    }

    pub fn get_location_by_id(&self, key: &str) -> Option<Point<f64>> {
        let mut items = self.items.lock().unwrap();

        if let Some(item) = items.get(key) {
            Some(item.clone())
        } else {
            None
        }
    }

}