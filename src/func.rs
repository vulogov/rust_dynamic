use crate::value::Value;
use crate::types::*;

impl Value {
    pub fn maybe(&self, appfn: MaybeFn) -> Self {
        if appfn(self) {
            return self.clone();
        }
        Value::none()
    }
    pub fn left_right(appfn: MaybeFn2, v1: &mut Value, v2: &mut Value) -> Self {
        if appfn(v1, v2) {
            return v1.dup().unwrap().regen_id().clone();
        }
        v2.dup().unwrap().regen_id().clone()
    }
}
