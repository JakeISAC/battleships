use crate::ships::ship_class::ShipClass;
use anyhow::Result;
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use std::ops::Not;

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

impl Not for Orientation {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Orientation::VERTICAL => Orientation::HORIZONTAL,
            Orientation::HORIZONTAL => Orientation::VERTICAL
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

// ships can only be orientated horizontally or vertically
// if a ship is vertical always move down (so from up till down),
// if horizontal always move right (so from left till right)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ship {
    class: ShipClass,
    fields: Vec<Status>,
    position: (Point, Point), // (start, end)
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
    ) -> Result<Self> {
        let mut fields: Vec<Status> = Vec::new();
        let position = {
            match orientation {
                // move down
                Orientation::VERTICAL => {
                    // calculate the end point based on ship size
                    let end = Point::new(start.x + size - 1, start.y);

                    // calculate the points representing the ship a.k.a modules
                    for i in start.x..=end.x {
                        fields.push(Status::OK(Point::new(i, start.y)));
                    }

                    (start, end)
                }
                // move right
                Orientation::HORIZONTAL => {
                    // calculate the end point based on ship size
                    let end = Point::new(start.x, start.y + size - 1);

                    // calculate the points representing the ship a.k.a modules
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

    pub fn try_hit(&mut self, hit_point: &Point) -> bool {
        // before hitting the ship needs to be zero
        match self.orientation {
            Orientation::HORIZONTAL => {
                if hit_point.x == self.position.0.x
                    && hit_point.y >= self.position.0.y // start
                    && hit_point.y <= self.position.1.y // end
                {
                    // find the position of the hit in the ship 'structure' array
                    let index = self.fields.iter().position(|x| {
                        if let Status::OK(point) = x {
                            return point.y == hit_point.y;
                        }
                        false
                    });

                    // update if module was not HIT before
                    if let Some(index) = index {
                        self.fields[index] = Status::HIT(hit_point.clone());
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
                    // find the position of the hit in the ship 'structure' array
                    let index = self.fields.iter().position(|x| {
                        if let Status::OK(point) = x {
                            return point.x == hit_point.x;
                        }
                        false
                    });

                    // update if module was not HIT before
                    if let Some(index) = index {
                        self.fields[index] = Status::HIT(hit_point.clone());
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
