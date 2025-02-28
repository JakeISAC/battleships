use crate::board::Board;
use crate::ship::{Orientation, Point, Ship};
use crate::ship_templates::submarine::Submarine;
use crate::ship_templates::template::ShipTemplate;
use itertools::Itertools;
use std::collections::HashMap;
use crate::ship_templates::carrier::Carrier;

mod ai;
mod board;
mod communication;
mod ship;
mod ship_templates;

/*
    TODO:
        Add a functionality where I ship gets to try both orientations in auto() mode.
        This needs to be done by including orientation as an argument to select_random_position().
        The orientation should be chosen at random in auto() function. So how it works it basically
        should try both and take the first one that succeeds.
 */


fn main() {
    let board_size = Board::new(5, 5);
    let mut board = board_size.get_board();

    let mut ships: Vec<Ship> = Vec::new();
    let mut occupied: HashMap<Orientation, Vec<Point>> = vec![
        (Orientation::HORIZONTAL, vec![]),
        (Orientation::VERTICAL, vec![]),
    ]
    .iter()
    .cloned()
    .collect();
    for _ in 0..100 {
        let ship = Carrier::auto(&board_size, &mut board, &occupied);
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
    ships.iter().for_each(|x| println!("{}", x));

    // let board = vec![vec![0; board_size.y]; board_size.x];
    // board.iter().for_each(|x| println!("{:?}", x));
}
