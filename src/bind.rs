use crate::value::Value;
use crate::types::*;

impl Value {
    pub fn bind(&mut self, appfn: AppFn) -> Self {
        return appfn(self.clone());
    }
    pub fn bind_values(appfn: AppFn2, value1: Value, value2: Value) -> Value {
        return appfn(value1, value2);
    }
}
