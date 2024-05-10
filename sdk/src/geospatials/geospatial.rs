use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use geo::{HaversineDistance, Point};
use crate::error::GeospatialError;

#[derive(Debug)]
pub struct Geospatial {
    items: Mutex<HashMap<String, Point<f64>>>,
}

pub trait Validatable<E> {
    fn try_create(longitude: f64, latitude: f64) -> Result<Point, E>;
}

impl Validatable<GeospatialError> for Point {

    fn try_create(longitude: f64, latitude: f64) -> Result<Point, GeospatialError> {
        if longitude > 180.0 || longitude < -180.0 {
            return Err(GeospatialError::LongitudeOutOfRange)
        }

        if latitude > 90.0 || latitude < -90.0 {
            return Err(GeospatialError::LatitudeOutOfRange)
        }

        Ok(Point::new(longitude, latitude))
    }
}

impl Geospatial {
    pub fn new() -> Arc<Self> {
        Arc::new(Geospatial {
            items: Mutex::new(HashMap::new()),
        })
    }

    pub fn upsert_location(&self, key: String, longitude: f64, latitude: f64) -> Result<(), GeospatialError> {
        let point_result =  Point::try_create(longitude, latitude);

        match point_result {
            Ok(point) => {
                let mut items = self.items.lock().unwrap();
                items.insert(key, point);
                Ok(())
            }
            Err(err) => Err(err)
        }
    }

    pub fn get_nearby_locations(&self, longitude: f64, latitude: f64, radius_in_meters: i32)
            -> Result<Vec<(String, Point<f64>)>, GeospatialError> {
        let central_point =  Point::try_create(longitude, latitude);
        if central_point.is_err() {
            return Err(central_point.err().unwrap())
        }

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