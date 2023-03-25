use crate::value::Value;
use crate::types::*;
use nanoid::nanoid;

impl Value {
    pub fn push(&mut self, value: Value) -> Self {
        if self.dt == LIST {
            let mut data: Vec<Value> = Vec::new();
            match &self.data {
                Val::List(v) => {
                    for i in v {
                        data.push(i.clone());
                    }
                    data.push(value.clone());
                }
                _ => {},
            }
            return Value::from_list(data);
        }
        let mut res = value.clone();
        res.q = self.q;
        res.id = nanoid!();
        return res;
    }
}
