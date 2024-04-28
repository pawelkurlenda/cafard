use geo::Point;

pub trait Validatable<E> {
    fn validate(&self) -> Result<(), E>;
    fn try_create() -> Result<Point, E>;
}