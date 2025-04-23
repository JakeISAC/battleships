use crate::board::Board;
use crate::game::game::Game;
use crate::ships::ship::Ship;
use anyhow::{anyhow, Result};
use colored::{Color, ColoredString, Colorize};
use regex::Regex;
use std::any::type_name;
use std::str::FromStr;

pub(crate) fn sanitize_and_transform<T: FromStr>(input: &str) -> Result<T> {
    let input = input.to_string();
    let text = input.trim();
    let re = Regex::new(r"\s+")?;
    let result = re.replace_all(text, " ");
    match result.parse::<T>() {
        Ok(res) => Ok(res),
        Err(_) => Err(anyhow!(
            "Could not parse the provided input: {} to type {}",
            input,
            type_name::<T>()
        )),
    }
}

pub fn matrix_with_colored_ships(board: &Board, ships: &Vec<Ship>) -> Vec<Vec<ColoredString>> {
    let mut colored_board: Vec<Vec<ColoredString>> = board
        .matrix_representation()
        .iter()
        .map(|x| x.iter().map(|y| y.white()).collect::<Vec<ColoredString>>())
        .collect();
    if ships.is_empty() {
        return colored_board;
    }
    for ship in ships {
        let modules = ship.get_fields();
        modules
            .iter()
            .for_each(|x| colored_board[x.x][x.y] = "*".color(ship.get_class().color()));
    }
    colored_board
}

pub fn color_specific_ship(
    ship_check: &Ship,
    board: &Board,
    ships: &Vec<Ship>,
    color: Color,
) -> Vec<Vec<ColoredString>> {
    let mut colored_board: Vec<Vec<ColoredString>> = board
        .matrix_representation()
        .iter()
        .map(|x| x.iter().map(|y| y.white()).collect::<Vec<ColoredString>>())
        .collect();
    if ships.is_empty() {
        return colored_board;
    }
    for ship in ships {
        if ship_check.get_position() == ship.get_position() {
            let modules = ship.get_fields();
            modules
                .iter()
                .for_each(|x| colored_board[x.x][x.y] = "*".color(color));
        } else {
            let modules = ship.get_fields();
            modules
                .iter()
                .for_each(|x| colored_board[x.x][x.y] = "*".color(Color::Red));
        }
    }
    colored_board
}

pub fn print_colored_matrix(matrix: Vec<Vec<ColoredString>>) {
    let y = matrix[0].len();
    print!("    ");
    for x in 0..y {
        print!("{:^4} ", x);
    }

    println!();

    for (i, row) in matrix.iter().enumerate() {
        print!("{:^3} ", i);
        for item in row {
            print!("{:^4} ", item);
        }
        println!();
    }
}

pub fn print_game(game: &Game) {
    print_colored_matrix(matrix_with_colored_ships(
        &game.get_board(),
        &game.get_ships(),
    ));
}
