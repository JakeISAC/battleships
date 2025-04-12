use crate::ships::ship::{Orientation, Point};
use as_any::AsAny;

pub(crate) trait Model: AsAny + Send {
    fn search(&self) -> Option<Point>;
    fn triangulation(&self, initial_point: Point) -> Orientation;
    fn attack(&self, initial_point: Point, hit_points: Vec<Point>);
}
