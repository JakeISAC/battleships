use crate::board::Board;
use crate::ships::ship::{Orientation, Point, Ship};
use crate::ships::ship_class::ShipClass;
use crate::ships::ship_class::ShipClass::{
    AircraftCarrier, Battleship, Destroyer, PatrolBoat, Submarine,
};
use crate::ships::template::ShipTemplate;
use crate::ui::util::{
    color_specific_ship, matrix_with_colored_ships, print_colored_matrix, sanitize_and_transform,
};
use anyhow::{anyhow, Result};
use colored::{Color, Colorize};
use std::collections::HashSet;
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
            "y" | "" => {
                return Ok((numeric_width, numeric_height));
            }
            _ => {
                Command::new("clear").status()?;
            }
        }
    }
}

pub fn get_ships_io(board: &Board) -> Result<Vec<Ship>> {
    println!("{}", "Note:
            Keep in mind the the program auto places ship based on orientation.
            Ship placed horizontally will be expanded to the right and ship places vertically
            will expand towards the bottom of the board.

            Please make sure that you 'start point' + 'size' does not collide with other ships on the board
            and that it does not extend beyond the board.".green()
    );
    println!(
        "{}",
        "Please press enter when ready to continue :).".white()
    );
    let mut answer: String = String::new();
    std::io::stdin().read_line(&mut answer)?;

    let mut board_vector = board.get_board();
    let mut occupied: HashSet<Point> = HashSet::new();
    let mut errors: Vec<String> = Vec::new();
    let mut iterator = 0usize;
    let mut ships: Vec<Ship> = Vec::new();
    while iterator != SHIPS.len() - 1 {
        Command::new("clear").status()?;
        errors.clear();

        let ship_class = SHIPS[iterator].clone();
        print_colored_matrix(matrix_with_colored_ships(&board, &ships));
        println!("-----------------------------------------");

        println!(
            "Placing '{}' with size {}. Ship {}/{}",
            ship_class,
            ship_class.size(),
            iterator + 1,
            NUM_SHIPS
        );

        println!();

        println!("Please enter ship's orientation (V or H) then x-coordinate then y-coordinate:");
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input)?;

        let try_parsed = parse_input(&input);
        let parsed = match try_parsed {
            Ok(res) => res,
            Err(e) => {
                println!("{}", "Your input failed to parse properly.".color(Color::BrightRed));
                errors.push(e.to_string());
                println!("{}", format!("{:?}", errors).red());
                println!("{}", "Please press enter when ready to continue.".color(Color::BrightRed));
                let mut answer: String = String::new();
                std::io::stdin().read_line(&mut answer)?;
                continue;
            }
        };
        let orientation: Orientation = sanitize_and_transform::<Orientation>(&parsed[0]).unwrap_or_else(|e| {
            errors.push(e.to_string());
            Orientation::VERTICAL
        });

        let x: usize = sanitize_and_transform::<usize>(&parsed[1]).unwrap_or_else(|e| {
            errors.push(e.to_string());
            0
        });

        let y: usize = sanitize_and_transform(&parsed[2]).unwrap_or_else(|e| {
            errors.push(e.to_string());
            0
        });

        if !errors.is_empty() {
            println!("{}", "Failed to create a new Ship. Try again!".color(Color::BrightRed));
            println!("{}", format!("{:?}", errors).red());
            println!("{}", "Please press enter when ready to continue.".color(Color::BrightRed));
            let mut answer: String = String::new();
            std::io::stdin().read_line(&mut answer)?;
            continue;
        }

        let start = Point::new(x, y);

        let ship = match ship_class.new(start, orientation, &mut board_vector, &occupied) {
            Ok(ship) => ship,
            Err(e) => {
                println!("{}", "Ship could not be created.".color(Color::BrightRed));
                errors.push(e.to_string());
                println!("{}", format!("{:?}", errors).red());
                println!("{}", "Please press enter when ready to continue.".color(Color::BrightRed));
                let mut answer: String = String::new();
                std::io::stdin().read_line(&mut answer)?;
                continue;
            }
        };
        ships.push(ship.clone());

        println!("-----------------------------------------");
        Command::new("clear").status()?;
        print_colored_matrix(color_specific_ship(&ship, &board, &ships, Color::Green));
        loop {
            println!("Are you happy with your choice?: ");
            println!("Yes [y] - continue, No [n] - try again");
            let mut answer: String = String::new();
            std::io::stdin().read_line(&mut answer)?;
            let parsed_answer = sanitize_and_transform::<String>(&answer).unwrap_or(String::new());
            match parsed_answer.to_lowercase().as_str() {
                "y" | "" => {
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
    println!("{}", "All ships have been placed. Please press enter to continue.".white());
    println!("{}", "Please press enter when ready to continue.".white());
    let mut answer: String = String::new();
    std::io::stdin().read_line(&mut answer)?;

    Ok(ships)
}

fn parse_input(input: &String) -> Result<Vec<String>> {
    let clean = sanitize_and_transform::<String>(input)?;
    let words: Vec<String> = clean.split(" ").map(|x| x.to_lowercase()).collect();
    if words.len() != 3 {
        return Err(anyhow!("More arguments than required"));
    }
    Ok(words)
}
