use crate::value::Value;
use crate::types::*;

impl Value {
    pub const fn from_float(value: f64) -> Self {
        Self {
            dt:   FLOAT,
            q:    100.0,
            data: Val::F64(value),
            attr: Vec::new(),
        }
    }
    pub const fn from_float32(value: f32) -> Self {
        Self {
            dt:   FLOAT,
            q:    100.0,
            data: Val::F64(value as f64),
            attr: Vec::new(),
        }
    }
    pub const fn from_int(value: i64) -> Self {
        Self {
            dt:   INTEGER,
            q:    100.0,
            data: Val::I64(value),
            attr: Vec::new(),
        }
    }
    pub const fn from_int32(value: i32) -> Self {
        Self {
            dt:   INTEGER,
            q:    100.0,
            data: Val::I64(value as i64),
            attr: Vec::new(),
        }
    }
    pub const fn from_bool(value: bool) -> Self {
        Self {
            dt:   BOOL,
            q:    100.0,
            data: Val::Bool(value),
            attr: Vec::new(),
        }
    }
}
