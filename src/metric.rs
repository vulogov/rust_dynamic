use serde::{Deserialize, Serialize};
use crate::value::{timestamp_ns};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metric {
    pub stamp:  u128,
    pub data:   f64,
}


impl Metric {
    pub fn new(value: f64) -> Self {
        Self {
            stamp:  timestamp_ns(),
            data:   value,
        }
    }
}
