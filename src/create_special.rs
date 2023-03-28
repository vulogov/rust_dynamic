use crate::value::{Value, timestamp_ms, timestamp_ns};
use nanoid::nanoid;
use crate::types::*;

impl Value {
    pub fn nodata() -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   NODATA,
            q:    100.0,
            data: Val::Null,
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn none() -> Self {
        Value::new()
    }
    pub fn pair(x: Value, y:Value) -> Self {
        Value::from_pair((x, y))
    }
    pub fn ptr(name: String, attrs: Vec<Value>) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   PTR,
            q:    100.0,
            data: Val::String(name),
            attr: attrs,
            curr: -1,
        }
    }
    pub fn call(name: String, attrs: Vec<Value>) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   CALL,
            q:    100.0,
            data: Val::String(name),
            attr: attrs,
            curr: -1,
        }
    }
    pub fn exit() -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   EXIT,
            q:    100.0,
            data: Val::Exit,
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn now() -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   TIME,
            q:    100.0,
            data: Val::F64(timestamp_ns()),
            attr: Vec::new(),
            curr: -1,
        }
    }
}
