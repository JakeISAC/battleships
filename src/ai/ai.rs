use crate::ai::model::Model;
use crate::board::Board;
use crate::game::game::Game;
use crate::protocol::protocol_commands::Command;
use crate::ships::ship::{Orientation, Point};
use crate::ships::ship_class::ShipClass;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};

type PossiblePlaces = Vec<Point>;
type ShipCount = i32;
type BoundingBox = Vec<Point>;
type AttackVector = (Vec<Point>, Vec<Point>);

type ConfirmationPoints = Vec<(Point, Point)>;

pub struct Isac {
    game: Game,
    search_grid: HashMap<ShipClass, (ShipCount, PossiblePlaces)>,
    limit: usize,
}

// setup
impl Isac {
    pub fn new(game: &Game, ships: &Vec<ShipClass>) -> Self {
        let board_set: HashSet<Point> = game.get_board().as_set();
        Self {
            game: game.clone(),
            search_grid: Self::generate_search_grid(ships, &board_set),
            limit: Self::find_limit(ships),
        }
    }

    /*
       Generate a search grid for the AI player. The search grid is calculated for each ShipClass,
       by calculating compliant points for all possible ships. The value fo the hashmap is a tuple of
       ship count and the arcs.

       For optimization purposes I aggregate the search points for repeating ShipClass.
    */
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

    /*
       Simple function to find what is the largest ship in the available set and use as a
       max for bounding all operations.
    */
    fn find_limit(ships: &Vec<ShipClass>) -> usize {
        let mut max = 0;
        for ship in ships {
            if ship.size() > max {
                max = ship.size();
            }
        }
        max
    }

    /*
       This function is an adjusted copy of compliant_points() found in ship template.

       Function that calculates all possible (compliant -- rules) start points for ships for
       both horizontal and vertical orientation in the set solver terminology this generates arcs
       for computing relations between ships.
    */
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

    /*
       Updated function for the search grid HashMap.

       Function simply updates arcs for all ships by removing the points that were hit on the
       opponent playing board.
    */
    fn update_search_grid(&mut self, points: Vec<&Point>) {
        self.search_grid
            .par_iter_mut()
            .for_each(|(_, (_, places))| {
                places.retain(|x| !points.contains(&x));
            });
    }

    /*
       Update function for the search grid HashMap.

       Function simply updates the ship count for an assumed ship class. I am guessing here since some
       ships have the same size.
    */
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

// gameplay
impl Isac {
    /*
       Function picks the next ShipClass for the search on the game board.
       Here we return a vector of PossiblePlaces for further investigation in the play() function.

       The search grid is sorted decremental for the number of PossiblePlaces per ShipClass,
       so the .next should always return the ship with next largest amount of searchable points.

       Search grid needs to be sorted on each search() call due to the fact that play() will updated
       PossiblePlaces during game play.

       This is because we want to reduce the domains of the rest of the ships as much as possible
       while searching.
    */
    pub fn search(&self) -> Option<(ShipClass, PossiblePlaces)> {
        let mut map: HashMap<ShipClass, (ShipCount, PossiblePlaces)> = self.search_grid
            .clone()
            .iter()
            .sorted_by(|(_, (_, places1)), (_, (_, places2))| places1.len().cmp(&places2.len()))
            .rev()
            .map(|(class, (count, places))| (class.clone(), (count.clone(), places.clone())))
            .collect();

        if let Some((key, value)) = map.iter().next() {
            let (_count, vector) = &value;
            return Some((key.clone(), vector.clone()));
        }
        None
    }

    pub fn triangulate(&self, initial_point: Point) -> Option<PossiblePlaces> {
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
        // if there is not bounding box for initial, simply return None
        None
    }

    pub fn find_orientation(&self, point_one: &Point, point_two: &Point) -> Option<Orientation> {
        let x_diff = point_one.x.abs_diff(point_two.x);
        let y_diff = point_one.y.abs_diff(point_two.y);
        if x_diff == 0 && y_diff <= self.limit && y_diff != 0 {
            Some(Orientation::VERTICAL)
        } else if y_diff == 0 && x_diff <= self.limit && x_diff != 0 {
            Some(Orientation::HORIZONTAL)
        } else {
            None
        }
    }

    pub fn attack(
        &self,
        hit_points: &Vec<Point>,
        orientation: Orientation,
    ) -> (ConfirmationPoints, AttackVector) {
        let offset = self.limit - hit_points.len();
        match orientation {
            // move along y-axis, x stays the same
            Orientation::HORIZONTAL => {
                let mut hits = hit_points.clone();
                hits.sort_by(|x, y| x.y.cmp(&y.y));

                let left_neighbours: Vec<Point> = Vec::new();
                let right_neighbours: Vec<Point> = Vec::new();

                if let Some(first) = hits.first() {
                    let last = hits.last().unwrap();
                    if first == last {
                        let range_left = 0..first.y;
                        let range_right = first.y..self.game.get_board().nr_columns();
                    } else {
                        todo!()
                    }
                }
                todo!()
            }
            // move along x-axis, y stays the same
            Orientation::VERTICAL => {
                todo!()
            }
        }
    }

    fn make_neighbours(point: &Point, board: &Board) {
        let mut left: Vec<Point> = Vec::new();
        let mut right: Vec<Point> = Vec::new();

        let range_left: Vec<usize> = (0..point.y).collect();
        let range_right = point.y..board.nr_columns();
    }
}

impl Model for Isac {
    fn play(&self, message_queue: Arc<Mutex<VecDeque<Box<dyn Command>>>>) {
        loop {
            while message_queue.lock().unwrap().is_empty() {
                continue;
            }
            // Code for actual game handling
            todo!()
        }
    }
}
