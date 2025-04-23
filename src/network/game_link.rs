use crate::protocol;
use crate::protocol::parser::parse;
use anyhow::Result;
use protocol::protocol_commands::Command;
use std::collections::VecDeque;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

pub fn game_handler(
    port: i32,
    message_queue: &mut Arc<Mutex<VecDeque<Box<dyn Command>>>>,
) -> Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Some(command) = handle_incoming(stream) {
                    let mut queue = message_queue
                        .lock()
                        .map_err(|e| anyhow::anyhow!("Mutex is poisoned: {:?}", e))?;
                    queue.push_back(command);
                }
            }
            Err(e) => eprintln!("{},", e),
        }
    }
    Ok(())
}

fn handle_incoming(mut stream: TcpStream) -> Option<Box<dyn Command>> {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(size) => {
            let packet = &buffer[..size];
            let message = String::from_utf8_lossy(packet).to_string();
            parse(message.as_str())
        }
        Err(_) => None,
    }
}
