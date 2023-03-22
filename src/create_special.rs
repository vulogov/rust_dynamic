use crate::value::Value;
use nanoid::nanoid;
use crate::types::*;

impl Value {
    pub fn nodata() -> Self {
        Self {
            id:   nanoid!(),
            dt:   NODATA,
            q:    100.0,
            data: Val::Null,
            attr: Vec::new(),
        }
    }
    pub fn none() -> Self {
        Value::new()
    }
    pub fn pair(x: Value, y:Value) -> Self {
        Value::from_pair((x, y))
    }
    pub fn ptr(name: String, attrs: Vec<Value>) -> Self {
        Self {
            id:   nanoid!(),
            dt:   PTR,
            q:    100.0,
            data: Val::String(name),
            attr: attrs,
        }
    }
    pub fn call(name: String, attrs: Vec<Value>) -> Self {
        Self {
            id:   nanoid!(),
            dt:   CALL,
            q:    100.0,
            data: Val::String(name),
            attr: attrs,
        }
    }
}
