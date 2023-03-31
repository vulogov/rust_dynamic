use crate::value::Value;
use crate::types::*;

impl Value {
    pub fn is_type(&self, t: u16) -> bool {
        self.dt == t
    }
    pub fn is_none(&self) -> bool {
        self.is_type(NONE)
    }
}
