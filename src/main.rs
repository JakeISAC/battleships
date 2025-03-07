use crate::board::Board;
use crate::ships::ship::{Point, Ship};
use crate::ships::ship_class::ShipClass;
use crate::ships::ship_class::ShipClass::{
    AircraftCarrier, Battleship, Destroyer, PatrolBoat, Submarine,
};
use crate::ships::template::ShipTemplate;
use crate::ui::manual_game::get_board_size_io;
use colored::Colorize;
use itertools::Itertools;
use std::collections::HashSet;
use crate::ui::util::{matrix_with_colored_ships, print_colored_matrix};

mod ai;
pub mod board;
mod communication;
mod game;
mod ships;
mod ui;

fn main() {
    let (width, height) = get_board_size_io().unwrap();
    let board = Board::new(width, height);
    // let ships = get_ships_io(&board).unwrap();
    // let game = Game::manual(&board, &ships);
    // let game = Game::auto(width, height);
    // print_game(&game);

    const SHIPS: [ShipClass; 15] = [
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
    for _ in 0..1 {
        for ship in SHIPS {
            let possible_ship = ship.auto(&board, &mut board_representation, &occupied);
            match possible_ship {
                Ok(ship) => {
                    occupied.extend(ship.get_fields());
                    ships.push(ship);
                }
                Err(e) => eprintln!("{}", e),
            }
        }
    }
    let end_time = chrono::Utc::now();
    println!(
        "It took {} seconds to successfully allocate {} ships on board {}x{}",
        (end_time - start_time).num_seconds(),
        ships.len(),
        board.get_x(),
        board.get_y(),
    );
     print_colored_matrix(matrix_with_colored_ships(&board, &ships));
}
