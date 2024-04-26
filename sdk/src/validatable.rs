use geo::Point;

pub trait Validatable<E> {
    fn validate(&self) -> Result<(), E>;
    fn valid_or_create() -> Result<Point, E>;
}