use crate::value::{Applicative, Value};
use crate::types::*;

impl Applicative {
    pub fn bind(&mut self, f: AppFn) -> Self {
        self.f = f;
        return self.clone();
    }
    pub fn apply(&self, value: Value) -> Value {
        let res = value.dup().unwrap().regen_id();
        return (self.f)(res);
    }
}
