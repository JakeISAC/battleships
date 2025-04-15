use crate::protocol::protocol_commands::Command;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub fn play(message_queue: Arc<Mutex<VecDeque<Box<dyn Command>>>>) {
    loop {
        while message_queue.lock().unwrap().is_empty() {
            continue;
        }
        // Code for actual game handling
        todo!()
    }
}
