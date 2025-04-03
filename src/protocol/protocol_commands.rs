use std::any::Any;

pub trait Command {
    fn to_string(&self) -> String;
}

/*
   # Game Request Protocol

   1. Request Coordinate:
     C:? - Ask for the coordinate being hit

   2. Request Game Lost:
     L:? - Ask if the other player lost

   3. Request Name:
     N:? - Ask for the other player's name

   4. Request Setup:
     SETUP:? - Ask if the other player finished setup
*/

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Requests {
    RequestCoordinate,
    RequestGameLost,
    RequestUserName,
    RequestSetup,
}

impl Command for Requests {
    fn to_string(&self) -> String {
        match self {
            Requests::RequestCoordinate => "C:?.".to_string(),
            Requests::RequestGameLost => "L:?.".to_string(),
            Requests::RequestUserName => "N:?.".to_string(),
            Requests::RequestSetup => "SETUP:?.".to_string(),
        }
    }
}

/*
   # Game Response Protocol

   1. Response Coordinate:
     C:<X>;<Y> - Example: C:10;10

   2. Response Coordinate List:
     C:<X1>;<Y1>;<X2>;<Y2>... - Example: C:1;2;3;4

   3. Response Game Lost:
     L:<STATUS> - <STATUS> = Y/N - Example: L:Y

   4. Response Hit:
     H:<NAME> - Hit
     M - Miss
     S:<NAME> - Sunk - Example: H:Destroyer, M, S:Battleship

   5. Response Name:
     N:<NAME> - Example: N:PlayerOne

   6. Response Null:
     NULL - No response ready

   7. Response Setup:
     SETUP:<STATUS> - <STATUS> = Y/N - Example: SETUP:Y
*/

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Responses<'a> {
    ResponseCoordinate(usize, usize),
    ResponseCoordinateList(Vec<(usize, usize)>),
    ResponseGameLost(&'a str),
    ResponseHit(&'a str, Option<&'a str>),
    ResponseUserName(&'a str),
    ResponseSetUp(&'a str),
    ResponseNull,
}

impl Command for Responses<'_> {
    fn to_string(&self) -> String {
        match self {
            Responses::ResponseCoordinate(x, y) => format!("C:{};{}", x, y),
            Responses::ResponseCoordinateList(list) => {
                let mut string_builder = String::from("C:");
                for (index, coordinate) in list.iter().enumerate() {
                    if index == list.len() - 1 {
                        let (x, y) = coordinate;
                        string_builder.push_str(&format!("{};{}.", x, y))
                    } else {
                        let (x, y) = coordinate;
                        string_builder.push_str(&format!("{};{};", x, y))
                    }
                }
                string_builder
            }
            Responses::ResponseGameLost(x) => format!("L:{}.", x.to_uppercase()),
            Responses::ResponseHit(command, ship_name) => {
                if let Some(name) = ship_name {
                    format!("{}:{}.", command, name)
                } else {
                    format!("{}.", command)
                }
            }
            Responses::ResponseUserName(name) => format!("N:{}.", name),
            Responses::ResponseSetUp(status) => format!("SETUP:{}.", status.to_uppercase()),
            Responses::ResponseNull => "NULL".to_string(),
        }
    }
}


/*
   # Game Acknowledgment Protocol

   1. Acknowledge Coordinate:
     ACK_C - Confirm receipt of coordinate

   2. Acknowledge Game Lost:
     ACK_L - Confirm receipt of game lost status

   3. Acknowledge Hit:
     ACK_H - Confirm receipt of hit response

   4. Acknowledge Name:
     ACK_N - Confirm receipt of player's name

   5. Acknowledge Setup:
     ACK_SETUP - Confirm receipt of setup message
*/

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Acknowledgements {
    AcknowledgementCoordinate,
    AcknowledgementGameLost,
    AcknowledgementHit,
    AcknowledgementUserName,
    AcknowledgementSetUp,
}

impl Command for Acknowledgements {
    fn to_string(&self) -> String {
        match self {
            Acknowledgements::AcknowledgementCoordinate => "ACK_C".to_string(),
            Acknowledgements::AcknowledgementGameLost => "ACK_L".to_string(),
            Acknowledgements::AcknowledgementHit => "ACK_H".to_string(),
            Acknowledgements::AcknowledgementUserName => "ACK_N".to_string(),
            Acknowledgements::AcknowledgementSetUp => "ACK_SETUP".to_string(),
        }
    }
}