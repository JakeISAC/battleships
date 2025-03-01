use crate::board::Board;
use std::fmt::{Display, Formatter};
use crate::ship::Ship;

pub enum GameMode {
    HUMAN,
    AI,
}

impl Display for GameMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameMode::AI => write!(f, "Human"),
            GameMode::HUMAN => write!(f, "AI"),
        }
    }
}

pub struct Game {
    board: Board,
    game_mode: GameMode,
    ships: Vec<Ship>,
}

impl Game {
    pub fn new(board: Board, game_mode: GameMode) -> Self {
        todo!()
    }
}
