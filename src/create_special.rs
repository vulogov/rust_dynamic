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
    pub fn result() -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   RESULT,
            q:    100.0,
            data: Val::List(Vec::new()),
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn to_result(value: Vec<Value>) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   RESULT,
            q:    100.0,
            data: Val::List(value),
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
            data: Val::Time(timestamp_ns()),
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn from_stamp(t: u128) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   TIME,
            q:    100.0,
            data: Val::Time(t),
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn queue() -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   QUEUE,
            q:    100.0,
            data: Val::Queue(Vec::new()),
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn to_queue(value: Vec<Value>) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   QUEUE,
            q:    100.0,
            data: Val::Queue(value),
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn fifo() -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   FIFO,
            q:    100.0,
            data: Val::Queue(Vec::new()),
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn to_fifo(value: Vec<Value>) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   FIFO,
            q:    100.0,
            data: Val::Queue(value),
            attr: Vec::new(),
            curr: -1,
        }
    }
}
