use colored::Colorize;
use crate::ships::template::ShipTemplate;
use itertools::Itertools;
use crate::board::Board;
use crate::game::Game;
use crate::ships::ship::{Orientation, Point, Ship};
use crate::ships::ship_class::ShipClass;
use crate::ui::manual_game::{get_board_size_io, get_ships_io};

mod ai;
mod communication;
mod game;
mod ships;
pub mod board;
mod ui;

fn main() {
    let board_size = get_board_size_io().unwrap();
    let board = Board::new(board_size.0, board_size.1);
    let ships = get_ships_io(&board);
    println!("{:?}", ships);
    // let board = vec![vec![0; board_size.y]; board_size.x];
    // board.iter().for_each(|x| println!("{:?}", x));
}
