use crate::value::Value;
use crate::types::*;

impl Value {
    pub const fn list() -> Self {
        Self {
            dt:   LIST,
            q:    100.0,
            data: Val::List(Vec::new()),
            attr: Vec::new(),
        }
    }
    pub const fn from_list(value: Vec<Value>) -> Self {
        Self {
            dt:   LIST,
            q:    100.0,
            data: Val::List(value),
            attr: Vec::new(),
        }
    }
}
