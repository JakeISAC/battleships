use crate::ai::model::Model;
use crate::board::Board;
use crate::game::game::Game;
use crate::ships::ship::{Orientation, Point};
use crate::ships::ship_class::ShipClass;

pub struct AiAgent {
    game: Game,
    opponent_board: Board,
    opponent_ships_left: Vec<ShipClass>
}

impl AiAgent {
    pub fn new(game: &Game, opponent_board: (usize, usize), ships_left: &Vec<ShipClass>) -> Self {
        Self {
            game: game.clone(),
            opponent_board: Board::new(opponent_board.0, opponent_board.1),
            opponent_ships_left: ships_left.clone()
        }
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