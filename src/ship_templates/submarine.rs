use std::collections::HashMap;
use crate::ship::{Orientation, Point, Ship, ShipClass};
use anyhow::{anyhow, Result};
use crate::board::Board;
use crate::ship_templates::template::ShipTemplate;

pub struct Submarine;

const SHIP_SIZE: usize = 3;

const SHIP: ShipClass = ShipClass::Submarine;

impl ShipTemplate for Submarine {
    fn new(start: Point, orientation: Orientation, board_size: &Board) -> Result<Ship> {
        Ship::new(SHIP, SHIP_SIZE, start, orientation, board_size)
    }

    fn auto(board_size: &Board, board: &mut Vec<Point>, occupied_places: &HashMap<Orientation, Vec<Point>>) -> Result<Ship> {
        let random_ship_loc = Self::select_random_position(SHIP_SIZE, board_size, board, occupied_places);
        if let Ok((start, orientation)) = random_ship_loc {
            return Ship::new(SHIP, SHIP_SIZE, start, orientation, board_size);
        }
        Err(anyhow!("Could not created {} in Auto mode.", SHIP))
    }
}