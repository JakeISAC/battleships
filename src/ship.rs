use crate::board::Board;
use anyhow::{anyhow, Result};
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShipClass {
    Destroyer,
    Submarine,
    Battleship,
    PatrolBoat,
    AircraftCarrier
}

impl Display for ShipClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ShipClass::Destroyer => write!(f, "Destroyer"),
            ShipClass::Submarine => write!(f, "Submarine"),
            ShipClass::Battleship => write!(f, "Battleship"),
            ShipClass::PatrolBoat => write!(f, "Patrol Boat"),
            ShipClass::AircraftCarrier => write!(f, "Aircraft Carrier")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Orientation {
    HORIZONTAL,
    VERTICAL,
}

impl Orientation {
    pub fn from_number(number: usize) -> Option<Self> {
        match number {
            0 => Some(Orientation::VERTICAL),
            1 => Some(Orientation::HORIZONTAL),
            _ => None
        }
    }
}

impl Display for Orientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Orientation::HORIZONTAL => write!(f, "Horizontal"),
            Orientation::VERTICAL => write!(f, "Vertical"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Status {
    HIT(Point),
    OK(Point),
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::HIT(x) => write!(f, "Hit ({},{})", x.x, x.y),
            Status::OK(x) => write!(f, "OK ({}, {})", x.x, x.y),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

// ship_templates can only be orientated horizontally or vertically
// if a ship is vertical always move down (so from up till down),
// if horizontal always move right (so from left till right)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ship {
    class: ShipClass,
    fields: Vec<Status>,
    position: (Point, Point), // hooks for the beginning and the end of the ship
    orientation: Orientation,
}

impl Display for Ship {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Ship => class: {}, fields: {:?}, orientation: {:?}, location: (start: {:?}, end: {:?})",
            self.class, self.fields, self.orientation, self.position.0, self.position.1
        )
    }
}

impl Ship {
    pub fn new(
        class: ShipClass,
        size: usize,
        start: Point,
        orientation: Orientation,
        board_size: &Board,
    ) -> Result<Self> {
        let mut fields: Vec<Status> = Vec::new();
        let position = {
            match orientation {
                // move down
                Orientation::VERTICAL => {
                    if start.x + size - 1 > board_size.x {
                        return Err(anyhow!(
                            "Can not place a ship. Ship size extends beyond the board."
                        ));
                    }
                    let end = Point::new(start.x + size - 1, start.y);

                    // calculate the points representing the ship
                    for i in start.x..=end.x {
                        fields.push(Status::OK(Point::new(i, start.y)));
                    }

                    (start, end)
                }
                // move right
                Orientation::HORIZONTAL => {
                    if start.y + size - 1 > board_size.y {
                        return Err(anyhow!(
                            "Can not place a ship. Ship size extends beyond the board."
                        ));
                    }
                    let end = Point::new(start.x, start.y + size - 1);

                    // calculate the points representing the ship
                    for i in start.y..=end.y {
                        fields.push(Status::OK(Point::new(start.x, i)));
                    }

                    (start, end)
                }
            }
        };
        Ok(Self {
            class,
            fields,
            position,
            orientation,
        })
    }

    pub fn hit(&mut self, hit_point: Point) -> bool {
        // before hitting the ship needs to be zero
        match self.orientation {
            Orientation::HORIZONTAL => {
                if hit_point.x == self.position.0.x
                    && hit_point.y >= self.position.0.y // start
                    && hit_point.y <= self.position.1.y // end
                {
                    // find if position of the hit in the ship 'structure' array
                    let index = self.fields.iter().position(|x| {
                        if let Status::OK(point) = x {
                            return point.y == hit_point.y;
                        }
                        false
                    });

                    // if it was not HIT already update
                    if let Some(index) = index {
                        self.fields[index] = Status::HIT(hit_point);
                        return true;
                    }
                }
                false
            }
            Orientation::VERTICAL => {
                if hit_point.y == self.position.0.y
                    && hit_point.x >= self.position.0.x // start
                    && hit_point.x <= self.position.1.x // end
                {
                    // find if position of the hit in the ship 'structure' array
                    let index = self.fields.iter().position(|x| {
                        if let Status::OK(point) = x {
                            return point.x == hit_point.x;
                        }
                        false
                    });

                    // if it was not HIT already update
                    if let Some(index) = index {
                        self.fields[index] = Status::HIT(hit_point);
                        return true;
                    }
                }
                false
            }
        }
    }

    pub fn is_sunk(&self) -> bool {
        self.fields.iter().all(|x| matches!(x, Status::HIT(_)))
    }

    pub fn get_class(&self) -> ShipClass {
        self.class.clone()
    }

    pub fn get_fields(&self) -> Vec<Point> {
        self.fields.clone().iter().filter_map(|status| match status {
            Status::HIT(point) |  Status::OK(point) => Some(point.clone()),
        }).collect()
    }

    pub fn get_position(&self) -> (Point, Point) {
        self.position.clone()
    }

    pub fn get_orientation(&self) -> Orientation {
        self.orientation.clone()
    }
}
