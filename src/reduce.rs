use crate::value::Value;
use crate::types::*;

impl Value {
    pub fn freduce(&mut self, appfn: AppFn2, value1: Value) -> Self {
        match self.dt {
            LIST => {
                match &self.data {
                    Val::List(v) => {
                        let mut v1 = value1;
                        for i in v {
                            v1 = appfn(v1, i.clone());
                        }
                        return v1;
                    }
                    _ => return appfn(value1, self.clone()),
                }
            }
            _ => {
                return appfn(value1, self.clone());
            }
        }
    }
}
