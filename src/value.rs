use serde::{Deserialize, Serialize};
use nanoid::nanoid;
use crate::types::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Value {
    pub id:     String,
    pub dt:     u16,
    pub q:      f64,
    pub data:   Val,
    pub attr:   Vec::<Value>,
}

impl Value {
    pub fn new() -> Self {
        Self {
            id:   nanoid!(),
            dt:   NONE,
            q:    0.0,
            data: Val::Null,
            attr: Vec::new(),
        }
    }
}
