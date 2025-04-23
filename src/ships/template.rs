use crate::board::Board;
use crate::ships::ship::{Orientation, Point, Ship};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use rand::Rng;
use rayon::prelude::*;
use std::collections::HashSet;

pub trait ShipTemplate {
    fn new(
        &self,
        start: Point,
        orientation: Orientation,
        board: &mut Vec<Point>,
        occupied_places: &HashSet<Point>,
    ) -> Result<Ship>
    where
        Self: Sized;
    fn auto(
        &self,
        board_size: &Board,
        board: &mut Vec<Point>,
        occupied_places: &HashSet<Point>,
    ) -> Result<Ship>
    where
        Self: Sized;

    fn select_random_position(
        ship_size: usize,
        orientation: &Orientation,
        board_size: &Board,
        board: &mut Vec<Point>,
        occupied_places: &HashSet<Point>,
    ) -> Result<Point>
    where
        Self: Sized,
    {
        let mut rng = rand::rng();
        match orientation {
            Orientation::VERTICAL => {
                let occupied = occupied_places;
                if !occupied.is_empty() {
                    let compliant_points =
                        Self::compliant_points(occupied, &orientation, &ship_size, board);
                    if let Some(options) = compliant_points {
                        let mut options = options.clone();
                        let mut random_point = rng.random_range(0..options.len());
                        while !options.is_empty() {
                            random_point = rng.random_range(0..options.len());
                            let chosen = options[random_point].clone();
                            let mut modules: Vec<Point> = Vec::new();
                            for i in 0..=ship_size {
                                modules.push(Point::new(chosen.x + i, chosen.y));
                            }
                            if modules.par_iter().all(|x| board.contains(x)) {
                                return Ok(options[random_point].clone());
                            }
                            options.remove(random_point);
                        }
                        return Err(anyhow!(
                            "No viable option for placing this ship Vertically was found."
                        ));
                    }
                    Err(anyhow!(
                        "There are no available spots on the board to place a ship of size {} Vertically.",
                        &ship_size
                    ))
                } else {
                    if board_size.nr_rows() - ship_size <= 0 || board_size.nr_columns() == 0 {
                        return Err(anyhow!("The Ship is to large to be placed on the board."));
                    }
                    let row = rng.random_range(0..board_size.nr_rows() - ship_size);
                    let column = rng.random_range(0..board_size.nr_columns());
                    Ok(Point::new(row, column))
                }
            }
            Orientation::HORIZONTAL => {
                let occupied = occupied_places;
                if !occupied.is_empty() {
                    let options = Self::compliant_points(occupied, &orientation, &ship_size, board);
                    if let Some(options) = options {
                        let mut options = options.clone();
                        let mut random_point = rng.random_range(0..options.len());
                        while !options.is_empty() {
                            random_point = rng.random_range(0..options.len());
                            let chosen = options[random_point].clone();
                            let mut modules: Vec<Point> = Vec::new();
                            for i in 0..=ship_size {
                                modules.push(Point::new(chosen.x, chosen.y + i));
                            }
                            if modules.par_iter().all(|x| board.contains(x)) {
                                return Ok(options[random_point].clone());
                            }
                            options.remove(random_point);
                        }
                        return Err(anyhow!(
                            "No viable option for placing this ship Horizontally was found."
                        ));
                    }
                    Err(anyhow!(
                        "There are no available spots on the board to place a ship of size {} Horizontally.",
                        &ship_size
                    ))
                } else {
                    if board_size.nr_columns() - ship_size <= 0 || board_size.nr_rows() == 0 {
                        return Err(anyhow!("The Ship is to large to be placed on the board."));
                    }
                    let row = rng.random_range(0..board_size.nr_rows());
                    let column = rng.random_range(0..board_size.nr_columns() - ship_size);
                    Ok(Point::new(row, column))
                }
            }
        }
    }

    fn compliant_points(
        occupied: &HashSet<Point>,
        orientation: &Orientation,
        ship_size: &usize,
        board: &mut Vec<Point>,
    ) -> Option<Vec<Point>>
    where
        Self: Sized,
    {
        // filter out available positions
        board.retain(|x| !occupied.contains(x));
        // create local tmp board as hashset look up
        let point_set: HashSet<&Point> = board.iter().collect();
        // return points that meet the criteria of orientation and ship size
        match orientation {
            Orientation::VERTICAL => {
                /*
                Board contains at least two points, where if there is a point (x, y) there also needs to be
                a point (x + ship_size, y)
                 */
                let possible: Vec<Point> = point_set
                    .par_iter()
                    .filter_map(|point| {
                        let target_point = Point {
                            x: point.x + ship_size - 1,
                            y: point.y,
                        };

                        if point_set.contains(&target_point) {
                            Some(point.clone())
                        } else {
                            None
                        }
                    })
                    .map(|x| x.clone())
                    .collect();

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
                let mut possible: Vec<Point> = point_set
                    .par_iter()
                    .filter_map(|point| {
                        let target_point = Point {
                            x: point.x,
                            y: point.y + ship_size - 1,
                        };

                        if point_set.contains(&target_point) {
                            Some(point.clone())
                        } else {
                            None
                        }
                    })
                    .map(|x| x.clone())
                    .collect();

                if possible.is_empty() {
                    return None;
                }
                Some(possible)
            }
        }
    }
}
