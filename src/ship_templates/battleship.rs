use crate::board::Board;
use crate::ship::{Orientation, Point, Ship, ShipClass};
use crate::ship_templates::template::ShipTemplate;
use anyhow::{anyhow, Result};
use rand::Rng;
use std::collections::HashMap;

pub struct Battleship;

const SHIP_SIZE: usize = 4;

const SHIP: ShipClass = ShipClass::Battleship;

impl ShipTemplate for Battleship {
    fn new(
        start: Point,
        orientation: Orientation,
        board: &mut Vec<Point>,
        occupied_places: &HashMap<Orientation, Vec<Point>>,
    ) -> Result<Ship> {
        let occupied = occupied_places.get(&orientation);
        if let Some(occupied) = occupied {
            let points = Self::compliant_points(occupied, &orientation, &SHIP_SIZE, board);
            if let Some(points) = points {
                if points.contains(&start) {
                    return Ship::new(SHIP, SHIP_SIZE, start, orientation);
                }
            }
            Err(anyhow!("Chosen point is not available."))
        } else {
            Ship::new(SHIP, SHIP_SIZE, start, orientation)
        }
    }

    fn auto(
        board_size: &Board,
        board: &mut Vec<Point>,
        occupied_places: &HashMap<Orientation, Vec<Point>>,
    ) -> Result<Ship> {
        let mut rng = rand::rng();
        let orientation = Orientation::from_number(rng.random_range(0..=1));
        if let Some(orientation) = orientation {
            let random_ship_loc = Self::select_random_position(
                SHIP_SIZE,
                &orientation,
                board_size,
                board,
                occupied_places,
            );
            if let Ok(start) = random_ship_loc {
                return Ship::new(SHIP, SHIP_SIZE, start, orientation);
            } else {
                let random_ship_loc = Self::select_random_position(
                    SHIP_SIZE,
                    &!orientation.clone(),
                    board_size,
                    board,
                    occupied_places,
                );
                if let Ok(start) = random_ship_loc {
                    return Ship::new(SHIP, SHIP_SIZE, start, !orientation);
                }
            }
        }
        Err(anyhow!("Could not created {} in Auto mode.", SHIP))
    }
}
