use crate::board::Board;
use crate::ships::ship::{Orientation, Point, Ship};
use crate::ships::template::ShipTemplate;
use anyhow::{anyhow, Result};
use rand::Rng;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use colored::Color;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShipClass {
    Destroyer,
    Submarine,
    Battleship,
    PatrolBoat,
    AircraftCarrier,
}

impl ShipClass {
    pub fn size(&self) -> usize {
        match self {
            ShipClass::Destroyer => 3,
            ShipClass::Submarine => 3,
            ShipClass::Battleship => 4,
            ShipClass::PatrolBoat => 2,
            ShipClass::AircraftCarrier => 5,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            ShipClass::Destroyer => Color::Blue,
            ShipClass::Submarine => Color::Red,
            ShipClass::Battleship => Color::Green,
            ShipClass::PatrolBoat => Color::Yellow,
            ShipClass::AircraftCarrier => Color::Magenta,
        }
    }
}

impl Display for ShipClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ShipClass::Destroyer => write!(f, "Destroyer"),
            ShipClass::Submarine => write!(f, "Submarine"),
            ShipClass::Battleship => write!(f, "Battleship"),
            ShipClass::PatrolBoat => write!(f, "Patrol Boat"),
            ShipClass::AircraftCarrier => write!(f, "Aircraft Carrier"),
        }
    }
}

impl ShipTemplate for ShipClass {
    fn new(
        &self,
        start: Point,
        orientation: Orientation,
        board: &mut Vec<Point>,
        occupied_places: &HashSet<Point>,
    ) -> Result<Ship> {
        if !occupied_places.is_empty() {
            // filter out available positions
            board.retain(|x| !occupied_places.contains(x));
            let try_ship = Ship::new(self.clone(), self.size(), start, orientation);
            if let Ok(ship) = &try_ship {
                let ship_modules = ship.get_fields();
                if ship_modules.iter().all(|x| board.contains(x)) {
                    return try_ship;
                }
            }
            Err(anyhow!("Chosen point is not available."))
        } else {
            let points = Self::compliant_points(&HashSet::new(), &orientation, &self.size(), board);
            if let Some(points) = points {
                if points.contains(&start) {
                    return Ship::new(self.clone(), self.size(), start, orientation);
                }
            }
            Err(anyhow!("Chosen point is not available."))
        }
    }

    fn auto(
        &self,
        board_size: &Board,
        board: &mut Vec<Point>,
        occupied_places: &HashSet<Point>,
    ) -> Result<Ship> {
        let mut rng = rand::rng();
        let orientation = Orientation::from_number(rng.random_range(0..=1));
        if let Some(orientation) = orientation {
            let random_ship_loc = Self::select_random_position(
                self.size(),
                &orientation,
                board_size,
                board,
                occupied_places,
            );
            if let Ok(start) = random_ship_loc {
                return Ship::new(self.clone(), self.size(), start, orientation);
            } else {
                let random_ship_loc = Self::select_random_position(
                    self.size(),
                    &!orientation.clone(),
                    board_size,
                    board,
                    occupied_places,
                );
                if let Ok(start) = random_ship_loc {
                    return Ship::new(self.clone(), self.size(), start, !orientation);
                }
            }
        }
        Err(anyhow!("Could not created {} in Auto mode.", self))
    }
}
