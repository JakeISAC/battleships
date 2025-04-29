use crate::protocol::protocol_commands::Command;
use as_any::AsAny;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub(crate) trait Model: AsAny + Send {
    fn play(&self, message_queue: Arc<Mutex<VecDeque<Box<dyn Command>>>>);
}
