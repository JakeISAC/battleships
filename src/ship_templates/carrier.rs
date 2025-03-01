use crate::board::Board;
use crate::ship::{Orientation, Point, Ship, ShipClass};
use crate::ship_templates::template::ShipTemplate;
use anyhow::{anyhow, Result};
use rand::Rng;
use std::collections::HashMap;

pub struct Carrier;

const SHIP_SIZE: usize = 5;

const SHIP: ShipClass = ShipClass::AircraftCarrier;

impl ShipTemplate for Carrier {
    fn new(start: Point, orientation: Orientation, board_size: &Board) -> Result<Ship> {
        Ship::new(SHIP, SHIP_SIZE, start, orientation, board_size)
    }

    fn auto(board_size: &Board, board: &mut Vec<Point>, occupied_places: &HashMap<Orientation, Vec<Point>>) -> Result<Ship> {
        let mut rng = rand::rng();
        let orientation = Orientation::from_number(rng.random_range(0..=1));
        if let Some(orientation) = orientation {
            let random_ship_loc = Self::select_random_position(SHIP_SIZE, &orientation, board_size, board, occupied_places);
            if let Ok(start) = random_ship_loc {
                return Ship::new(SHIP, SHIP_SIZE, start, orientation, board_size);
            } else {
                match &orientation {
                    Orientation::VERTICAL => {
                        let random_ship_loc = Self::select_random_position(SHIP_SIZE, &Orientation::HORIZONTAL, board_size, board, occupied_places);
                        if let Ok(start) = random_ship_loc {
                            return Ship::new(SHIP, SHIP_SIZE, start, orientation, board_size);
                        }
                    },
                    Orientation::HORIZONTAL => {
                        let random_ship_loc = Self::select_random_position(SHIP_SIZE, &Orientation::VERTICAL, board_size, board, occupied_places);
                        if let Ok(start) = random_ship_loc {
                            return Ship::new(SHIP, SHIP_SIZE, start, orientation, board_size);
                        }
                    }
                }
            }
        }
        Err(anyhow!("Could not created {} in Auto mode.", SHIP))
    }
}