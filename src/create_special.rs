use crate::value::Value;
use crate::types::*;

impl Value {
    pub const fn nodata() -> Self {
        Self {
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
}
