use crate::value::Value;
use crate::error::BundError;
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
    pub const fn from_string(value: String) -> Self {
        Self {
            dt:   STRING,
            q:    100.0,
            data: Val::String(value),
            attr: Vec::new(),
        }
    }
    pub fn from_str(value: &str) -> Self {
        Self {
            dt:   STRING,
            q:    100.0,
            data: Val::String(value.to_string()),
            attr: Vec::new(),
        }
    }
    pub fn from_bin(value: Vec<u8>) -> Self {
        Self {
            dt:   BIN,
            q:    100.0,
            data: Val::Binary(value),
            attr: Vec::new(),
        }
    }
    pub fn from_error(value: BundError) -> Self {
        Self {
            dt:   ERROR,
            q:    100.0,
            data: Val::Error(value),
            attr: Vec::new(),
        }
    }
}
