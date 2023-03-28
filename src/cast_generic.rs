use std::any::Any;
use crate::value::Value;

impl Value {
    pub fn as_any(&self) -> &dyn Any {
        self
    }
}
