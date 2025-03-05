use crate::board::Board;
use crate::game::Game;
use crate::ships::ship::Point;
use crate::ships::template::ShipTemplate;
use crate::ui::manual_game::{get_board_size_io, get_ships_io};
use crate::ui::util::{board_with_colored_ships, print_colored_matrix};
use colored::{Color, Colorize};
use itertools::Itertools;

mod ai;
pub mod board;
mod communication;
mod game;
mod ships;
mod ui;

fn main() {
    let board_size = get_board_size_io().unwrap();
    let board = Board::new(board_size.0, board_size.1);
    let ships = get_ships_io(&board);
    if let Ok(ships) = ships {
        let mut game = Game::manual(&board, &ships);
        game.hit(&Point::new(0, 1));
        game.hit(&Point::new(0, 2));
        game.hit(&Point::new(3, 5));
        print_colored_matrix(board_with_colored_ships(&game.get_board(), &ships, Color::Blue));
    }
    // let board = vec![vec![0; board_size.y]; board_size.x];
    // board.iter().for_each(|x| println!("{:?}", x));
}
