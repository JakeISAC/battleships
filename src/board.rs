use crate::ships::ship::Point;
use itertools::Itertools;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Clone)]
pub struct Board {
    nr_rows: usize,
    nr_columns: usize,
    points: Vec<Point>,
}

impl Board {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            nr_rows: rows,
            nr_columns: columns,
            points: Self::generate_board(rows, columns),
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
        for _ in 0..self.nr_rows {
            matrix.push(vec![".".to_string(); self.nr_columns])
        }
        matrix
    }

    pub fn as_vec(&self) -> Vec<Point> {
        self.points.clone()
    }

    pub fn as_set(&self) -> HashSet<Point> {
        self.points.clone().iter().map(|x| x.clone()).collect()
    }

    pub fn contains(&self, point: &Point) -> bool {
        self.points.contains(point)
    }

    pub fn nr_rows(&self) -> usize {
        self.nr_rows.clone()
    }

    pub fn nr_columns(&self) -> usize {
        self.nr_columns.clone()
    }
}
