use crate::ships::ship::Point;
use itertools::Itertools;

#[derive(Clone)]
pub struct Board {
    x: usize,
    y: usize,
    points: Vec<Point>,
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

    pub fn matrix_representation(&self) -> Vec<Vec<String>> {
        let mut matrix: Vec<Vec<String>> = Vec::new();
        for i in 0..self.x {
            matrix.push(vec![".".to_string(); self.y])
        }
        matrix
    }

    pub fn get_board(&self) -> Vec<Point> {
        self.points.clone()
    }

    pub fn get_x(&self) -> usize {
        self.x.clone()
    }

    pub fn get_y(&self) -> usize {
        self.y.clone()
    }
}
