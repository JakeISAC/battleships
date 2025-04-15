use crate::ai::model::Model;
use crate::game::game::Game;
use crate::ships::ship::{Orientation, Point};
use crate::ships::ship_class::ShipClass;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

type PossiblePlaces = HashSet<Point>;

pub struct AiAgent {
    game: Game,
    search_grid: HashMap<ShipClass, PossiblePlaces>,
}

impl AiAgent {
    pub fn new(game: &Game, ships: &Vec<ShipClass>) -> Self {
        let board_set: HashSet<Point> = game.get_board().as_set();
        Self {
            game: game.clone(),
            search_grid: Self::generate_search_grid(ships, &board_set),
        }
    }

    fn generate_search_grid(
        ship_classes: &Vec<ShipClass>,
        board: &HashSet<Point>,
    ) -> HashMap<ShipClass, PossiblePlaces> {
        ship_classes
            .par_iter()
            .filter_map(|x| {
                if let Some(possible) = Self::compliant_points(x.size(), board) {
                    return Some((x.clone(), possible));
                }
                None
            })
            .collect::<HashMap<ShipClass, PossiblePlaces>>()
    }

    fn compliant_points(ship_size: usize, board: &HashSet<Point>) -> Option<PossiblePlaces>
    where
        Self: Sized,
    {
        // return points that meet the criteria of orientation and ship size
        // consider vertical orientation
        let possible: HashSet<Point> = board
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
}

impl Model for AiAgent {
    fn search(&self) -> Option<Point> {
        todo!()
    }

    fn triangulation(&self, initial_point: Point) -> Orientation {
        todo!()
    }

    fn attack(&self, initial_point: Point, hit_points: Vec<Point>) {
        todo!()
    }
}
