use crate::ships::ship::{Orientation, Point};
use as_any::AsAny;
use crate::ships::ship_class::ShipClass;

type BoundingBox = Vec<Point>;

pub(crate) trait Model: AsAny + Send {
    fn search(&self) -> Option<(ShipClass, Point)>;
    fn triangulate(&self, initial_point: Point) -> Option<BoundingBox>;
    fn find_orientation(&self, point_one: &Point, point_two: &Point) -> Option<Orientation>;
    fn attack(&self, initial_point: Point, hit_points: Vec<Point>);
}
