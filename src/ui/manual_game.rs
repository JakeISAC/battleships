use crate::board::Board;
use crate::ships::ship::{Orientation, Point, Ship};
use crate::ships::ship_class::ShipClass;
use crate::ships::ship_class::ShipClass::{
    AircraftCarrier, Battleship, Destroyer, PatrolBoat, Submarine,
};
use crate::ships::template::ShipTemplate;
use crate::ui::util::{
    board_with_colored_ships, color_specific_ship, print_colored_matrix, sanitize_and_transform,
};
use anyhow::{anyhow, Result};
use colored::{Color, Colorize};
use std::collections::{HashMap, HashSet};
use std::process::Command;

const NUM_SHIPS: usize = 15;

const SHIPS: [ShipClass; NUM_SHIPS] = [
    AircraftCarrier,
    Battleship,
    Battleship,
    Destroyer,
    Destroyer,
    Destroyer,
    Submarine,
    Submarine,
    Submarine,
    Submarine,
    PatrolBoat,
    PatrolBoat,
    PatrolBoat,
    PatrolBoat,
    PatrolBoat,
];

type Width = usize;
type Height = usize;
pub fn get_board_size_io() -> Result<(Width, Height)> {
    loop {
        let mut errors: Vec<String> = Vec::new();
        println!("Please enter width of the board:");
        let mut width: String = String::new();
        std::io::stdin().read_line(&mut width)?;
        let numeric_width = sanitize_and_transform::<usize>(&width).unwrap_or_else(|e| {
            errors.push(e.to_string());
            0
        });
        println!("Please enter height of the board:");
        let mut height: String = String::new();
        std::io::stdin().read_line(&mut height)?;
        let numeric_height = sanitize_and_transform::<usize>(&height).unwrap_or_else(|e| {
            errors.push(e.to_string());
            0
        });

        if !errors.is_empty() {
            println!("Failed to get Width and Height.");
            println!("{:?}", errors);
            println!("Please press enter to retry.");
            let mut answer: String = String::new();
            std::io::stdin().read_line(&mut answer)?;
            Command::new("clear").status()?;
            continue;
        }

        println!("Are you happy with your choice?: ");
        println!("Yes [y] - continue, No [n] - try again");
        let mut answer: String = String::new();
        std::io::stdin().read_line(&mut answer)?;
        let parsed_answer = sanitize_and_transform::<String>(&answer).unwrap_or(String::new());
        match parsed_answer.to_lowercase().as_str() {
            "y" => {
                return Ok((numeric_width, numeric_height));
            },
            _ => {
                Command::new("clear").status()?;
            }
        }
    }
}

pub fn get_ships_io(board: &Board) -> Result<Vec<Ship>> {
    println!("{}", "Note: \n
            Keep in mind the the program auto places ship based on orientation.
            Please make sure that you 'start point' + 'size' does not collide with other ships on the board
            and that it does not extend beyond the board.".green()
    );
    println!("Please press enter when ready to continue :).");
    let mut answer: String = String::new();
    std::io::stdin().read_line(&mut answer)?;

    let mut board_vector = board.get_board();
    let mut occupied: HashSet<Point> = HashSet::new();
    let mut errors: Vec<String> = Vec::new();
    let mut iterator = 0usize;
    let mut ships: Vec<Ship> = Vec::new();
    while iterator != SHIPS.len() - 1 {
        Command::new("clear").status()?;
        let ship_class = SHIPS[iterator].clone();
        print_colored_matrix(board_with_colored_ships(&board, &ships, Color::Red));
        println!("-----------------------------------------");

        println!(
            "Placing '{}' with size {}. Ship {}/{}",
            ship_class,
            ship_class.size(),
            iterator + 1,
            NUM_SHIPS
        );

        println!();

        println!("Please enter ship's orientation (V or H):");
        let mut orientation: String = String::new();
        std::io::stdin().read_line(&mut orientation)?;
        let string_orientation =
            sanitize_and_transform::<String>(&orientation).unwrap_or_else(|e| {
                errors.push(e.to_string());
                String::new()
            });
        let parsed_orientation: Orientation = match string_orientation.to_lowercase().as_str() {
            "v" => Orientation::VERTICAL,
            "h" => Orientation::HORIZONTAL,
            _ => {
                println!("Failed to parse Orientation. Try again!");
                continue;
            }
        };

        println!("Please enter x-coordinate:");
        let mut start_x: String = String::new();
        std::io::stdin().read_line(&mut start_x)?;
        let numeric_x = sanitize_and_transform::<usize>(&start_x).unwrap_or_else(|e| {
            errors.push(e.to_string());
            0
        });
        println!("Please enter y-coordinate:");
        let mut start_y: String = String::new();
        std::io::stdin().read_line(&mut start_y)?;
        let numeric_y = sanitize_and_transform::<usize>(&start_y).unwrap_or_else(|e| {
            errors.push(e.to_string());
            0
        });

        if !errors.is_empty() {
            println!("Failed to create a new Ship. Try again!");
            errors.iter().for_each(|x| println!("{}", x));
            continue;
        }

        let start = Point::new(numeric_x, numeric_y);

        let ship = match ship_class.new(start, parsed_orientation, &mut board_vector, &occupied) {
            Ok(ship) => ship,
            Err(e) => {
                println!("{}", e.to_string());
                continue;
            }
        };
        ships.push(ship.clone());

        println!("-----------------------------------------");
        print_colored_matrix(color_specific_ship(&ship, &board, &ships, Color::Green));
        loop {
            println!("Are you happy with your choice?: ");
            println!("Yes [y] - continue, No [n] - try again");
            let mut answer: String = String::new();
            std::io::stdin().read_line(&mut answer)?;
            let parsed_answer = sanitize_and_transform::<String>(&answer).unwrap_or(String::new());
            match parsed_answer.to_lowercase().as_str() {
                "y" => {
                    occupied.extend(ship.get_fields());
                    iterator += 1;
                    break;
                }
                "n" => {
                    let _ = ships.pop();
                    break;
                }
                _ => {}
            }
        }
    }
    println!("All ships have been placed. Please press enter to continue.");
    println!("Please press enter when ready to continue.");
    let mut answer: String = String::new();
    std::io::stdin().read_line(&mut answer)?;

    Ok(ships)
}
