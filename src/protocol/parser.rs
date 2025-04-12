use crate::protocol::protocol_commands::Acknowledgements::{
    AcknowledgementCoordinate, AcknowledgementGameLost, AcknowledgementHit, AcknowledgementSetUp,
    AcknowledgementUserName,
};
use crate::protocol::protocol_commands::Requests::{
    RequestCoordinate, RequestGameLost, RequestSetup, RequestUserName,
};
use crate::protocol::protocol_commands::Responses::{
    ResponseCoordinate, ResponseCoordinateList, ResponseGameLost, ResponseHit, ResponseNull,
    ResponseSetUp, ResponseUserName,
};
use crate::protocol::protocol_commands::Command;
use crate::protocol::sanitize::sanitize;
use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

pub fn parse(input: &str) -> Option<Box<dyn Command>> {
    // make sure input meets the protocol requirements
    let san_input = sanitize(input);
    let input = san_input.as_str();

    // check for NULL first to avoid necessary regex
    if input == "NULL" {
        return Some(Box::from(ResponseNull));
    }

    let request_pattern = Regex::new(r"^(C|L|N|SETUP):\?\.$").unwrap();
    if request_pattern.find(input).is_some() {
        return match input {
            "C:?." => Some(Box::from(RequestCoordinate)),
            "L:?." => Some(Box::from(RequestGameLost)),
            "N:?." => Some(Box::from(RequestUserName)),
            "SETUP:?." => Some(Box::from(RequestSetup)),
            _ => None,
        };
    }

    let ack_pattern = Regex::new(r"^ACK_(C|L|H|N|SETUP)$").unwrap();
    if ack_pattern.find(input).is_some() {
        return match input {
            "ACK_C" => Some(Box::from(AcknowledgementCoordinate)),
            "ACK_L" => Some(Box::from(AcknowledgementGameLost)),
            "ACK_H" => Some(Box::from(AcknowledgementHit)),
            "ACK_N" => Some(Box::from(AcknowledgementUserName)),
            "ACK_SETUP" => Some(Box::from(AcknowledgementSetUp)),
            _ => None,
        };
    }

    let game_lost = Regex::new(r"^L:(Y|N)\.$").unwrap();
    if game_lost.find(input).is_some() {
        return match input {
            "L:Y." => Some(Box::from(ResponseGameLost("Y".to_string()))),
            "L:N." => Some(Box::from(ResponseGameLost("N".to_string()))),
            _ => None,
        };
    }

    let response_hit_h = Regex::new(r"^H:(\S+)\.$").unwrap();
    if response_hit_h.find(input).is_some() {
        let capture = response_hit_h.captures(input).unwrap();
        let name = capture.get(1);
        if let Some(name) = name {
            return Some(Box::from(ResponseHit(
                "H".to_string(),
                Some(name.as_str().to_string()),
            )));
        }
        return None;
    }

    let response_hit_s = Regex::new(r"^S:(\S+)\.$").unwrap();
    if response_hit_s.find(input).is_some() {
        let capture = response_hit_s.captures(input).unwrap();
        let name = capture.get(1);
        if let Some(name) = name {
            return Some(Box::from(ResponseHit(
                "S".to_string(),
                Some(name.as_str().to_string()),
            )));
        }
        return None;
    }

    let response_hit_m = Regex::new(r"^M\.$").unwrap();
    if response_hit_m.find(input).is_some() {
        return Some(Box::from(ResponseHit("M".to_string(), None)));
    }

    let response_user_name = Regex::new(r"^N:(\S+)\.$").unwrap();
    if response_user_name.find(input).is_some() {
        let capture = response_user_name.captures(input).unwrap();
        let name = capture.get(1);
        if let Some(name) = name {
            return Some(Box::from(ResponseUserName(name.as_str().to_string())));
        }
        return None;
    }

    let response_setup = Regex::new(r"^SETUP:(\S+)\.$").unwrap();
    if response_setup.find(input).is_some() {
        let capture = response_setup.captures(input).unwrap();
        let stat = capture.get(1);
        if let Some(stat) = stat {
            return Some(Box::from(ResponseSetUp(stat.as_str().to_string())));
        }
        return None;
    }

    // If none of the one before match we need to parse out a response
    let response_coordinate = Regex::new(r"^C:(\d+);(\d+)\.$").unwrap();
    if response_coordinate.find(input).is_some() {
        let capture = response_coordinate.captures(input).unwrap();
        let x = capture.get(1);
        let y = capture.get(2);
        if let Some(x) = x {
            if let Some(y) = y {
                let x_numeric: usize = match x.as_str().parse() {
                    Ok(val) => val,
                    Err(e) => {
                        eprintln!("{}", e);
                        return None;
                    }
                };
                let y_numeric: usize = match y.as_str().parse() {
                    Ok(val) => val,
                    Err(e) => {
                        eprintln!("{}", e);
                        return None;
                    }
                };
                return Some(Box::from(ResponseCoordinate(x_numeric, y_numeric)));
            }
        }
        return None;
    }

    let response_coordinate_list = Regex::new(r"^C:([\d;]+)\.$").unwrap();
    if response_coordinate_list.find(input).is_some() {
        let capture = response_coordinate_list.captures(input).unwrap();
        let array = capture.get(1);
        if let Some(array) = array {
            let numeric: Vec<usize> = array
                .as_str()
                .split(";")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<usize>())
                .filter(|x| x.is_ok())
                .map(|x| x.unwrap())
                .collect();
            let pairs: Vec<(usize, usize)> = numeric
                .iter()
                .step_by(2)
                .zip(numeric.iter().skip(1).step_by(2))
                .map(|(x, y)| (x.clone(), y.clone()))
                .collect();
            return Some(Box::from(ResponseCoordinateList(pairs)));
        }
        return None;
    }

    None
}
