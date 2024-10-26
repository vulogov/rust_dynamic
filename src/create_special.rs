use crate::value::{Value, timestamp_ms, timestamp_ns};
use std::collections::HashMap;
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
            tags:   HashMap::new(),
        }
    }
    pub fn none() -> Self {
        Value::new()
    }
    pub fn context() -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   CONTEXT,
            q:    100.0,
            data: Val::String(nanoid!()),
            attr: Vec::new(),
            curr: -1,
            tags:   HashMap::new(),
        }
    }
    pub fn named_context(name: String) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   CONTEXT,
            q:    100.0,
            data: Val::String(name),
            attr: Vec::new(),
            curr: -1,
            tags:   HashMap::new(),
        }
    }
    pub fn text_buffer(name: String) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   TEXTBUFFER,
            q:    100.0,
            data: Val::String(name),
            attr: Vec::new(),
            curr: -1,
            tags:   HashMap::new(),
        }
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
            tags:   HashMap::new(),
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
            tags:   HashMap::new(),
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
            tags:   HashMap::new(),
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
            tags:   HashMap::new(),
        }
    }
    pub fn operator(opcode: i32, param1: Value, param2: Value) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   OPERATOR,
            q:    100.0,
            data: Val::Operator(Operator{opcode: opcode, opvalue1: param1.to_binary().unwrap(), opvalue2: param2.to_binary().unwrap()}),
            attr: Vec::new(),
            curr: -1,
            tags:   HashMap::new(),
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
            tags:   HashMap::new(),
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
            tags:   HashMap::new(),
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
            tags:   HashMap::new(),
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
            tags:   HashMap::new(),
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
            tags:   HashMap::new(),
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
            tags:   HashMap::new(),
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
            tags:   HashMap::new(),
        }
    }
    pub fn json(value: serde_json::Value) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   JSON,
            q:    100.0,
            data: Val::Json(value),
            attr: Vec::new(),
            curr: -1,
            tags:   HashMap::new(),
        }
    }
    pub fn json_array() -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   JSON,
            q:    100.0,
            data: Val::Json(serde_json::json!([])),
            attr: Vec::new(),
            curr: -1,
            tags:   HashMap::new(),
        }
    }
    pub fn json_object() -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   JSON,
            q:    100.0,
            data: Val::Json(serde_json::json!({})),
            attr: Vec::new(),
            curr: -1,
            tags:   HashMap::new(),
        }
    }
    pub fn json_wrap(data: String) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   JSON_WRAPPED,
            q:    100.0,
            data: Val::String(data),
            attr: Vec::new(),
            curr: -1,
            tags:   HashMap::new(),
        }
    }
}
