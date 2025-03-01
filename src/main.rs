use crate::board::Board;
use crate::ship::{Orientation, Point, Ship};
use crate::ship_templates::carrier::Carrier;
use crate::ship_templates::template::ShipTemplate;
use chrono::Utc;
use itertools::Itertools;
use std::collections::HashMap;
use crate::ship_templates::submarine::Submarine;

mod ai;
mod board;
mod communication;
mod ship;
mod ship_templates;
mod game;

fn main() {
    let board_size = Board::new(3, 3);
    let mut board = board_size.get_board();

    let mut ships: Vec<Ship> = Vec::new();
    let mut occupied: HashMap<Orientation, Vec<Point>> = vec![
        (Orientation::HORIZONTAL, vec![]),
        (Orientation::VERTICAL, vec![]),
    ]
    .iter()
    .cloned()
    .collect();
    let time_start = Utc::now();
    for _ in 0..6 {
        let ship = Submarine::auto(&board_size, &mut board, &occupied);
        match ship {
            Ok(ship) => {
                ships.push(ship.clone());
                occupied
                    .entry(ship.get_orientation())
                    .or_insert_with(Vec::new)
                    .extend(ship.get_fields());
            }
            Err(e) => {
                println!("{}", e.to_string());
                continue;
            }
        }
    }
    let time_end = Utc::now();
    ships.iter().for_each(|x| println!("{}", x));
    println!(
        "Entire operation took {} seconds, for game board size {}x{}",
        (time_end - time_start).num_seconds(),
        board_size.x,
        board_size.y
    );

    // let board = vec![vec![0; board_size.y]; board_size.x];
    // board.iter().for_each(|x| println!("{:?}", x));
}
