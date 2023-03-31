use nanoid::nanoid;
use crate::value::{Value, timestamp_ms};
use crate::error::BundError;
use crate::types::*;

impl Value {
    pub fn from_float(value: f64) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   FLOAT,
            q:    100.0,
            data: Val::F64(value),
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn from_float32(value: f32) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   FLOAT,
            q:    100.0,
            data: Val::F64(value as f64),
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn from_int(value: i64) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   INTEGER,
            q:    100.0,
            data: Val::I64(value),
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn from_int32(value: i32) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   INTEGER,
            q:    100.0,
            data: Val::I64(value as i64),
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn from_bool(value: bool) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   BOOL,
            q:    100.0,
            data: Val::Bool(value),
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn from_string<N: AsRef<str>>(value: N) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   STRING,
            q:    100.0,
            data: Val::String(value.as_ref().to_string()),
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn from_str(value: &str) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   STRING,
            q:    100.0,
            data: Val::String(value.to_string()),
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn from_bin(value: Vec<u8>) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   BIN,
            q:    100.0,
            data: Val::Binary(value),
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn make_envelope(value: Vec<u8>) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   ENVELOPE,
            q:    100.0,
            data: Val::Binary(value),
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn binary() -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   BIN,
            q:    100.0,
            data: Val::Binary(Vec::new()),
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn from_error(value: BundError) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   ERROR,
            q:    100.0,
            data: Val::Error(value),
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn from_pair(value: (Value, Value)) -> Self {
        let mut res = Value::from_list(vec![value.0, value.1]);
        res.dt = PAIR;
        res
    }
    pub fn from_timestamp(value: u128) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   TIME,
            q:    100.0,
            data: Val::Time(value),
            attr: Vec::new(),
            curr: -1,
        }
    }
}
