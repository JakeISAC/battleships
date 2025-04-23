use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use crate::ships::ship::{Orientation, Point};
use crate::ships::ship_class::ShipClass;
use as_any::AsAny;
use crate::protocol::protocol_commands::Command;

type BoundingBox = Vec<Point>;
type AttackVector = Vec<Point>;

pub(crate) trait Model: AsAny + Send {
    fn play(&self, message_queue: Arc<Mutex<VecDeque<Box<dyn Command>>>>);
}
