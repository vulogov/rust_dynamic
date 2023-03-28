use crate::value::{Value, timestamp_ms};
use nanoid::nanoid;
use crate::types::*;

impl Value {
    pub fn list() -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   LIST,
            q:    100.0,
            data: Val::List(Vec::new()),
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn from_list(value: Vec<Value>) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   LIST,
            q:    100.0,
            data: Val::List(value),
            attr: Vec::new(),
            curr: -1,
        }
    }
}
