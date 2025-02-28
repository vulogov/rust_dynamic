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
            tags:   HashMap::new(),
        }
    }
    pub fn valuemap() -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   VALUEMAP,
            q:    100.0,
            data: Val::ValueMap(HashMap::new()),
            attr: Vec::new(),
            curr: -1,
            tags:   HashMap::new(),
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
            tags:   HashMap::new(),
        }
    }
    pub fn from_valuemap(value: HashMap<Value, Value>) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   VALUEMAP,
            q:    100.0,
            data: Val::ValueMap(value),
            attr: Vec::new(),
            curr: -1,
            tags:   HashMap::new(),
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
            tags:   HashMap::new(),
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
            tags:   HashMap::new(),
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
            tags:   HashMap::new(),
        }
    }
    pub fn conditional() -> Self {
        let mut data: HashMap<String, Value> = HashMap::new();
        data.insert("type".to_string(), Value::from_string("unknown"));
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   CONDITIONAL,
            q:    100.0,
            data: Val::Map(data),
            attr: Vec::new(),
            curr: -1,
            tags:  HashMap::new(),
        }
    }
    pub fn conditional_of_type(t: String) -> Self {
        let mut data: HashMap<String, Value> = HashMap::new();
        data.insert("type".to_string(), Value::from_string(t));
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   CONDITIONAL,
            q:    100.0,
            data: Val::Map(data),
            attr: Vec::new(),
            curr: -1,
            tags:  HashMap::new(),
        }
    }
    pub fn conditional_from_dict(value: HashMap<String, Value>) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   CONDITIONAL,
            q:    100.0,
            data: Val::Map(value),
            attr: Vec::new(),
            curr: -1,
            tags:   HashMap::new(),
        }
    }
    pub fn make_class() -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   CLASS,
            q:    100.0,
            data: Val::Map(HashMap::new()),
            attr: Vec::new(),
            curr: -1,
            tags:   HashMap::new(),
        }
    }
    pub fn make_object() -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   OBJECT,
            q:    100.0,
            data: Val::Map(HashMap::new()),
            attr: Vec::new(),
            curr: -1,
            tags:   HashMap::new(),
        }
    }
}
