use crate::value::Value;
use nanoid::nanoid;
use crate::types::*;

impl Value {
    pub fn list() -> Self {
        Self {
            id:   nanoid!(),
            dt:   LIST,
            q:    100.0,
            data: Val::List(Vec::new()),
            attr: Vec::new(),
        }
    }
    pub fn from_list(value: Vec<Value>) -> Self {
        Self {
            id:   nanoid!(),
            dt:   LIST,
            q:    100.0,
            data: Val::List(value),
            attr: Vec::new(),
        }
    }
}
