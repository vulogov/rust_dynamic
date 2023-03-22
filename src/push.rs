use crate::value::Value;
use crate::types::*;

impl Value {
    pub fn map(&mut self, appfn: AppFn) -> Self {
        if self.dt == LIST {
            let mut data: Vec<Value> = Vec::new();
            match &self.data {
                Val::List(v) => {
                    for i in v {
                        data.push(appfn(i.clone()));
                    }
                }
                _ => {},
            }
            return Value::from_list(data);
        }
        return appfn(self.clone());
    }
}
