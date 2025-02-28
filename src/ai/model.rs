use crate::ship::{Orientation, Point};

pub(crate) trait Model {
    fn search() -> Option<Point>;
    fn triangulation(initial_point: Point) -> Orientation;
    fn attack(initial_point: Point, hit_points: Vec<Point>);
}