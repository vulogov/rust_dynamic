use crate::value::Value;
use crate::types::*;

impl Value {
    pub fn bind(&mut self, appfn: AppFn) -> Self {
        return appfn(self.clone());
    }
}
