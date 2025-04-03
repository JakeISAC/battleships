use std::collections::HashSet;
use std::fs::File;
use crate::board::Board;
use crate::game::game::Game;
use crate::ships::template::ShipTemplate;
use crate::ui::manual_game::{get_board_size_io, get_ships_io};
use colored::Colorize;
use itertools::Itertools;
use rayon::prelude::{IntoParallelIterator, ParallelExtend};
use crate::protocol::parser::parse;
use crate::ships::ship::{Point, Ship};
use crate::ships::ship_class::ShipClass;
use crate::ships::ship_class::ShipClass::{AircraftCarrier, Battleship, Destroyer, PatrolBoat, Submarine};
use crate::ui::util::print_game;
use std::io::Write;

mod ai;
pub mod board;
mod game;
mod ships;
mod ui;
mod protocol;

fn main() {
    let mut output = File::create("result.txt").expect("Failed");

    let (width, height) = get_board_size_io().unwrap();
    let board = Board::new(width, height);

    let SHIPS: [ShipClass; 15] = [
        AircraftCarrier,
        Battleship,
        Battleship,
        Destroyer,
        Destroyer,
        Destroyer,
        Submarine,
        Submarine,
        Submarine,
        Submarine,
        PatrolBoat,
        PatrolBoat,
        PatrolBoat,
        PatrolBoat,
        PatrolBoat,
    ];

    let mut ships: Vec<Ship> = Vec::new();
    let mut board_representation = board.get_board();
    let mut occupied: HashSet<Point> = HashSet::new();
    let start_time = chrono::Utc::now();
    for _ in 0..150 {
        for ship in &SHIPS {
            let possible_ship = ship.auto(&board, &mut board_representation, &occupied);
            match possible_ship {
                Ok(ship) => {
                    occupied.par_extend(ship.get_fields());
                    ships.push(ship);
                }
                Err(e) => eprintln!("{}", e),
            }
        }
    }
    let end_time = chrono::Utc::now();
    let line = format!(
        "It took {} seconds to successfully allocate {} ships on board {}x{}",
        (end_time - start_time).num_seconds(),
        ships.len(),
        board.get_width(),
        board.get_height(),
    ).to_string();

    write!(output, "{}", line).expect("TODO: panic message");
}

