use crate::value::{Value, timestamp_ms};
use std::collections::HashMap;
use nanoid::nanoid;
use crate::types::*;

impl Value {
    pub fn lambda() -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   LAMBDA,
            q:    100.0,
            data: Val::Lambda(Vec::new()),
            attr: Vec::new(),
            curr: -1,
            tags:   HashMap::new(),
        }
    }
    pub fn to_lambda(value: Vec<Value>) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   LAMBDA,
            q:    100.0,
            data: Val::Lambda(value),
            attr: Vec::new(),
            curr: -1,
            tags:   HashMap::new(),
        }
    }
}
