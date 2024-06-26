use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use nanoid::nanoid;
use crate::types::*;

pub fn timestamp_ms() -> f64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as f64
}

pub fn timestamp_ns() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u128
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Value {
    pub id:     String,
    pub stamp:  f64,
    pub dt:     u16,
    pub q:      f64,
    pub data:   Val,
    pub attr:   Vec::<Value>,
    pub curr:   i32,
    pub tags:   HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct Applicative {
    pub f:      AppFn,
}

impl Value {
    pub fn new() -> Self {
        Self {
            id:     nanoid!(),
            stamp:  timestamp_ms(),
            dt:     NONE,
            q:      0.0,
            data:   Val::Null,
            attr:   Vec::new(),
            curr:   -1,
            tags:   HashMap::new(),
        }
    }
}

impl Applicative {
    pub fn new(f: AppFn) -> Self {
        Self {
            f:     f,
        }
    }
}
