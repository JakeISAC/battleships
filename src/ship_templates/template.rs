use crate::board::Board;
use crate::ship::{Orientation, Point, Ship};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use rand::Rng;
use std::collections::{HashMap, HashSet};

pub trait ShipTemplate {
    fn new(
        start: Point,
        orientation: Orientation,
        board: &mut Vec<Point>,
        occupied_places: &HashMap<Orientation, Vec<Point>>,
    ) -> Result<Ship>;
    fn auto(
        board_size: &Board,
        board: &mut Vec<Point>,
        occupied_places: &HashMap<Orientation, Vec<Point>>,
    ) -> Result<Ship>;

    fn select_random_position(
        ship_size: usize,
        orientation: &Orientation,
        board_size: &Board,
        board: &mut Vec<Point>,
        occupied_places: &HashMap<Orientation, Vec<Point>>,
    ) -> Result<Point> {
        let mut rng = rand::rng();
        match orientation {
            Orientation::VERTICAL => {
                let occupied = occupied_places.get(&Orientation::VERTICAL).unwrap();
                if !occupied.is_empty() {
                    let options = Self::compliant_points(occupied, &orientation, &ship_size, board);
                    if let Some(options) = options {
                        let random_point = rng.random_range(0..options.len());
                        return Ok(options[random_point].clone());
                    }
                    Err(anyhow!(
                        "There are no available spots on the board to place a ship of size {} Vertically.",
                        &ship_size
                    ))
                } else {
                    if board_size.x - ship_size <= 0 || board_size.y == 0 {
                        return Err(anyhow!("The Ship is to large to be placed on the board."));
                    }
                    let row = rng.random_range(0..board_size.x - ship_size);
                    let column = rng.random_range(0..board_size.y);
                    Ok(Point::new(row, column))
                }
            }
            Orientation::HORIZONTAL => {
                let occupied = occupied_places.get(&Orientation::HORIZONTAL).unwrap();
                if !occupied_places.is_empty() {
                    let options = Self::compliant_points(occupied, &orientation, &ship_size, board);
                    if let Some(options) = options {
                        let random_point = rng.random_range(0..options.len());
                        return Ok(options[random_point].clone());
                    }
                    Err(anyhow!(
                        "There are no available spots on the board to place a ship of size {} Horizontally.",
                        &ship_size
                    ))
                } else {
                    if board_size.y - ship_size <= 0 || board_size.x == 0 {
                        return Err(anyhow!("The Ship is to large to be placed on the board."));
                    }
                    let row = rng.random_range(0..board_size.x);
                    let column = rng.random_range(0..board_size.y - ship_size);
                    Ok(Point::new(row, column))
                }
            }
        }
    }

    fn compliant_points(
        occupied: &Vec<Point>,
        orientation: &Orientation,
        ship_size: &usize,
        board: &mut Vec<Point>,
    ) -> Option<Vec<Point>> {
        // filter out available positions
        board.retain(|x| !occupied.contains(x));
        // create local tmp board
        let tmp_board = board.clone();
        // return points that meet the criteria of orientation and ship size
        match orientation {
            Orientation::VERTICAL => {
                /*
                Board contains at least two points, where if there is a point (x, y) there also needs to be
                a point (x + ship_size, y)
                 */
                let mut possible: Vec<Point> = Vec::new();
                let point_set: HashSet<&Point> = tmp_board.iter().collect();
                for point in &tmp_board {
                    let target_point = Point {
                        x: point.x + ship_size - 1,
                        y: point.y,
                    };

                    if point_set.contains(&target_point) {
                        possible.push(point.clone());
                    }
                }

                if possible.is_empty() {
                    return None;
                }
                Some(possible)
            }
            Orientation::HORIZONTAL => {
                /*
                Board contains at least two points, where if there is a point (x, y) there also needs to be
                a point (x, y + ship_size)
                 */
                let mut possible: Vec<Point> = Vec::new();
                let point_set: HashSet<&Point> = tmp_board.iter().collect();
                for point in &tmp_board {
                    let target_point = Point {
                        x: point.x,
                        y: point.y + ship_size - 1,
                    };

                    if point_set.contains(&target_point) {
                        possible.push(point.clone());
                    }
                }

                if possible.is_empty() {
                    return None;
                }
                Some(possible)
            }
        }
    }
}
