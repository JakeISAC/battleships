use crate::board::Board;
use crate::ships::ship::{Point, Ship};
use crate::ships::ship_class::ShipClass;
use crate::ships::ship_class::ShipClass::{
    AircraftCarrier, Battleship, Destroyer, PatrolBoat, Submarine,
};
use crate::ships::template::ShipTemplate;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fmt::Display;

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

#[derive(Clone)]
pub struct Game {
    board: Board,
    ships: Vec<Ship>,
}

impl Game {
    pub fn auto(width: usize, height: usize) -> Self {
        let board = Board::new(width, height);
        let mut board_representation = board.as_vec();
        let mut occupied: HashSet<Point> = HashSet::new();

        let mut ships: Vec<Ship> = Vec::new();
        for x in SHIPS {
            let ship = x.auto(&board, &mut board_representation, &occupied);
            match ship {
                Ok(ship) => {
                    occupied.par_extend(ship.get_fields());
                    ships.push(ship);
                }
                Err(e) => {
                    println!("{}", e.to_string());
                    continue;
                }
            }
        }

        Self { board, ships }
    }

    pub fn manual(board: &Board, ships: &Vec<Ship>) -> Self {
        Self {
            board: board.clone(),
            ships: ships.clone(),
        }
    }

    // return (success hit, was the ship sunk)
    pub fn hit(&mut self, point: &Point) -> (bool, bool) {
        for ship in &mut self.ships {
            if ship.try_hit(point) {
                return (true, ship.is_sunk());
            }
        }
        (false, false)
    }

    // check if all ships are sunken
    pub fn game_over(&self) -> bool {
        self.ships.iter().all(|x| x.is_sunk())
    }

    pub fn get_board(&self) -> Board {
        self.board.clone()
    }

    pub fn get_board_as_ref(&self) -> &Board {
        &self.board
    }

    pub fn get_ships(&self) -> Vec<Ship> {
        self.ships.clone()
    }
}
