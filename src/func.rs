use crate::value::Value;
use crate::types::*;

impl Value {
    pub fn maybe(&self, appfn: MaybeFn) -> Self {
        if appfn(self) {
            return self.clone();
        }
        Value::none()
    }
}
