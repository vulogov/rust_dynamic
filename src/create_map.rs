use std::collections::hash_map::HashMap;
use crate::value::{Value, timestamp_ms};
use nanoid::nanoid;
use crate::types::*;

impl Value {
    pub fn dict() -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   MAP,
            q:    100.0,
            data: Val::Map(HashMap::new()),
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn from_dict(value: HashMap<String, Value>) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   MAP,
            q:    100.0,
            data: Val::Map(value),
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn info() -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   INFO,
            q:    100.0,
            data: Val::Map(HashMap::new()),
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn config() -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   CONFIG,
            q:    100.0,
            data: Val::Map(HashMap::new()),
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn association<N: AsRef<str>>(name: N, value: Value) -> Self {
        let mut data: HashMap<String, Value> = HashMap::new();
        data.insert(name.as_ref().to_string(), value);
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   CONFIG,
            q:    100.0,
            data: Val::Map(data),
            attr: Vec::new(),
            curr: -1,
        }
    }
}
