use crate::value::{Value, timestamp_ms};
use std::collections::HashMap;
use nanoid::nanoid;
use crate::types::*;

impl Value {
    pub fn matrix() -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   MATRIX,
            q:    100.0,
            data: Val::Matrix(Vec::new()),
            attr: Vec::new(),
            curr: -1,
            tags:   HashMap::new(),
        }
    }
    pub fn from_matrix(value: Vec<Vec<Value>>) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   MATRIX,
            q:    100.0,
            data: Val::Matrix(value),
            attr: Vec::new(),
            curr: -1,
            tags:   HashMap::new(),
        }
    }
}
