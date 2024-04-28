use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use geo::{HaversineDistance, Point};
use crate::error::GeospatialError;
use crate::validatable::Validatable;

#[derive(Debug)]
pub struct Geospatial {
    items: Mutex<HashMap<String, Point<f64>>>,
}

pub trait Validation {
    fn valid();
}

impl Validation for Point {
    fn valid() {
        todo!()
    }
}

pub trait Validatable<E> {
    fn validate(&self) -> Result<(), E>;
    fn try_create(longitude: f64, latitude: f64) -> Result<Point, E>;
}

impl Validatable<GeospatialError> for Point {
    fn validate(&self) -> Result<(), GeospatialError> {
        todo!()
    }

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

    // TODO: upsert_location_2 vs upsert_location
    pub fn upsert_location_2(&self, key: String, point: Point) {
        point.valid();

        //let a_1 = Point::valid();
        let b = point.validate();
        let a_2 = Point::try_create();
        let a_3 = Point::valid();

        let mut items = self.items.lock().unwrap();

        items.insert(key, point);
    }

    pub fn upsert_location(&self, key: String, longitude: f64, latitude: f64) {
        let point =  Point::new(longitude, latitude);

        let mut items = self.items.lock().unwrap();

        items.insert(key, point);
    }

    pub fn get_nearby_locations(&self, central_point: &Point, radius_in_meters: i32) -> Vec<(String, Point<f64>)> {// TODO: consider change type to  Vec<(String, (f64, f64))>
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