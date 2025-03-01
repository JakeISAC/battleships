use crate::ship::Point;
use itertools::Itertools;

pub struct Board {
    pub x: usize,
    pub y: usize,
    pub points: Vec<Point>,
}

impl Board {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            points: Self::generate_board(x, y),
        }
    }

    fn generate_board(x: usize, y: usize) -> Vec<Point> {
        // create board
        let mut x: Vec<usize> = (0..x).collect();
        let y: Vec<usize> = (0..y).collect();
        let mut board: Vec<Point> = x
            .into_iter()
            .cartesian_product(y)
            .map(|(x, y)| Point::new(x, y))
            .collect();
        board
    }

    pub fn get_board(&self) -> Vec<Point> {
        self.points.clone()
    }
}
