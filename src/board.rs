use crate::ships::ship::Point;
use itertools::Itertools;

#[derive(Clone)]
pub struct Board {
    width: usize,
    height: usize,
    points: Vec<Point>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            points: Self::generate_board(width, height),
        }
    }

    fn generate_board(x: usize, y: usize) -> Vec<Point> {
        // create board
        let x: Vec<usize> = (0..x).collect();
        let y: Vec<usize> = (0..y).collect();
        let board: Vec<Point> = x
            .into_iter()
            .cartesian_product(y)
            .map(|(x, y)| Point::new(x, y))
            .collect();
        board
    }

    pub fn matrix_representation(&self) -> Vec<Vec<String>> {
        let mut matrix: Vec<Vec<String>> = Vec::new();
        for _ in 0..self.width {
            matrix.push(vec![".".to_string(); self.height])
        }
        matrix
    }

    pub fn get_board(&self) -> Vec<Point> {
        self.points.clone()
    }

    pub fn get_width(&self) -> usize {
        self.width.clone()
    }

    pub fn get_height(&self) -> usize {
        self.height.clone()
    }
}
