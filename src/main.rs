use crate::network::game_link::game_handler;
use crate::protocol::protocol_commands::Command;
use crate::ships::template::ShipTemplate;
use colored::Colorize;
use itertools::Itertools;
use rayon::prelude::{IntoParallelIterator, ParallelExtend};
use std::collections::VecDeque;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

mod ai;
pub mod board;
mod game;
mod network;
mod protocol;
mod ships;
mod ui;

use anyhow::Result;

fn main() -> Result<()> {
    let port = 1332;
    let mut queue = Arc::new(Mutex::new(VecDeque::new()));
    let mut threaded_queue = Arc::clone(&queue);
    thread::spawn(move || {
        let _ = game_handler(port, &mut threaded_queue);
    });

    loop {
        queue
            .lock()
            .unwrap()
            .iter()
            .for_each(|x| println!("{}", x.to_string()));
        sleep(Duration::from_secs(2));
    }

    Ok(())
}
