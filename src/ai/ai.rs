use crate::ai::model::Model;
use crate::game::game::Game;
use crate::ships::ship::{Orientation, Point};
use crate::ships::ship_class::ShipClass;
use itertools::Itertools;
use rand::Rng;
use rayon::prelude::*;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

type PossiblePlaces = Vec<Point>;
type ShipCount = i32;

pub struct AiAgent {
    game: Game,
    search_grid: HashMap<ShipClass, (ShipCount, PossiblePlaces)>,
    limit: usize
}

impl AiAgent {
    pub fn new(game: &Game, ships: &Vec<ShipClass>) -> Self {
        let board_set: HashSet<Point> = game.get_board().as_set();
        Self {
            game: game.clone(),
            search_grid: Self::generate_search_grid(ships, &board_set),
            limit: Self::find_limit(ships)
        }
    }

    fn generate_search_grid(
        ship_classes: &Vec<ShipClass>,
        board: &HashSet<Point>,
    ) -> HashMap<ShipClass, (ShipCount, PossiblePlaces)> {
        let mut grid: HashMap<ShipClass, (ShipCount, PossiblePlaces)> = HashMap::new();
        for ship in ship_classes {
            match grid.entry(ship.clone()) {
                Entry::Occupied(mut occupied) => {
                    let (count, _) = occupied.get_mut();
                    *count += 1;
                }
                Entry::Vacant(mut vacant) => {
                    if let Some(possible) = Self::compliant_points(ship.size(), board) {
                        vacant.insert((1, possible));
                    }
                }
            }
        }
        grid
    }

    fn find_limit(ships: &Vec<ShipClass>) -> usize {
        let mut max = 0;
        for ship in ships {
            if ship.size() > max {
                max = ship.size();
            }
        }
        max
    }

    fn compliant_points(ship_size: usize, board: &HashSet<Point>) -> Option<PossiblePlaces>
    where
        Self: Sized,
    {
        // return points that meet the criteria of orientation and ship size
        // consider vertical orientation
        let possible: Vec<Point> = board
            .par_iter()
            .filter_map(|point| {
                let target_point_vertical = Point {
                    x: point.x + ship_size - 1,
                    y: point.y,
                };

                let target_point_horizontal = Point {
                    x: point.x,
                    y: point.y + ship_size - 1,
                };

                if board.contains(&target_point_vertical)
                    || board.contains(&target_point_horizontal)
                {
                    Some(point.clone())
                } else {
                    None
                }
            })
            .collect();

        if possible.is_empty() {
            return None;
        }

        Some(possible)
    }

    fn update_search_grid(&mut self, point: &Point) {
        self.search_grid
            .par_iter_mut()
            .for_each(|(_, (_, places))| {
                places.retain(|x| x != point);
            });
    }

    fn update_ship_sunk(&mut self, ship: &ShipClass) {
        match self.search_grid.entry(ship.clone()) {
            Entry::Occupied(mut occupied) => {
                let (count, _) = occupied.get_mut();
                if *count - 1 > 0 {
                    *count -= 1;
                } else {
                    let _ = self.search_grid.remove(ship);
                }
            }
            _ => {}
        }
    }
}

impl Model for AiAgent {
    fn search(&self) -> Option<(ShipClass, Point)> {
        if let Some((key, value)) = self.search_grid.iter().next() {
            let mut rng = rand::rng();
            let vector = &value.1;
            let point = rng.random_range(0..vector.len());
            if let Some(point) = vector.get(point) {
                return Some((key.clone(), point.clone()));
            }
        }
        None
    }

    fn triangulate(&self, initial_point: Point) -> Option<Vec<Point>> {
        // calculate the initial a.k.a simple bounding box for a ship
        let mut bounding_box = initial_point.simple_bounding_box(&self.game.get_board());
        // filter found simple points against search grid to determine weather they are available
        // if a point is an `island` it should return None
        if let Some(bounding_box) = bounding_box {
            let available_points: Vec<Point> = bounding_box
                .par_iter()
                .filter(|x| {
                    self.search_grid
                        .par_iter()
                        .any(|(_, (_, places))| places.contains(x))
                })
                .cloned()
                .collect();
            if available_points.is_empty() {
                return None;
            }
            return Some(available_points);
        }
        // if there are no initial, simple points return None
        None
    }

    fn find_orientation(&self, point_one: &Point, point_two: &Point) -> Option<Orientation> {
        let x_diff = point_one.x.abs_diff(point_two.x);
        let y_diff = point_one.y.abs_diff(point_two.y);
        if x_diff == 0 && y_diff <= self.limit && y_diff != 0 {
            Some(Orientation::VERTICAL)
        } else if x_diff <= self.limit && x_diff != 0 && y_diff == 0 {
            Some(Orientation::HORIZONTAL)
        } else {
            None
        }
    }

    fn attack(&self, initial_point: Point, hit_points: Vec<Point>) {
        todo!()
    }
}
