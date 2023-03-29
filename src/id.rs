use crate::value::{Value, timestamp_ms};
use nanoid::nanoid;

impl Value {
    pub fn regen_id(&mut self) -> Self {
        self.id = nanoid!();
        self.stamp = timestamp_ms();
        self.clone()
    }
}
