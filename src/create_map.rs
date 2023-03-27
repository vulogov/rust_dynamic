use std::collections::hash_map::HashMap;
use crate::value::Value;
use nanoid::nanoid;
use crate::types::*;

impl Value {
    pub fn dict() -> Self {
        Self {
            id:   nanoid!(),
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
            dt:   CONFIG,
            q:    100.0,
            data: Val::Map(HashMap::new()),
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn association(name: String, value: Value) -> Self {
        let mut data: HashMap<String, Value> = HashMap::new();
        data.insert(name, value);
        Self {
            id:   nanoid!(),
            dt:   CONFIG,
            q:    100.0,
            data: Val::Map(data),
            attr: Vec::new(),
            curr: -1,
        }
    }
}
