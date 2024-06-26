use crate::databases::error::DatabaseError;

pub struct CreateIndexRequest {
    pub is_unique: bool,
    pub fields: Vec<CreateIndexFieldRequest>
}

pub struct CreateIndexFieldRequest {
    pub field_name: String,
    pub order: CreateIndexFieldOrderRequest
}

#[derive(Debug)]
pub enum CreateIndexFieldOrderRequest {
    ASC,
    DSC
}

impl CreateIndexRequest {
    fn valid() -> Result<(), DatabaseError> {


        Ok(())
    }
}

/*pub trait Validate<T> {
    fn valid(&self) -> Result<T, DatabaseError>;
}

pub trait Validatable<E> {
    fn try_create(longitude: f64, latitude: f64) -> Result<Point, E>;
}

impl crate::geospatials::geospatial::Validatable<GeospatialError> for Point {

    fn try_create(longitude: f64, latitude: f64) -> Result<Point, GeospatialError> {
        if longitude > 180.0 || longitude < -180.0 {
            return Err(GeospatialError::LongitudeOutOfRange)
        }

        if latitude > 90.0 || latitude < -90.0 {
            return Err(GeospatialError::LatitudeOutOfRange)
        }

        Ok(Point::new(longitude, latitude))
    }
}*/