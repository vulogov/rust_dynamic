use nanoid::nanoid;
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
        let mut res = self.clone();
        res.id = nanoid!();
        return appfn(res);
    }
    pub fn map_float(&mut self, appfn: FloatFn) -> Self {
        if self.dt == LIST {
            let mut data: Vec<Value> = Vec::new();
            match &self.data {
                Val::List(v) => {
                    for i in v {
                        if i.dt == FLOAT {
                            data.push(Value::from_float(appfn(i.cast_float().unwrap())));
                        }
                    }
                }
                _ => {},
            }
            return Value::from_list(data);
        }
        if self.dt == FLOAT {
            let mut res = self.clone();
            res.id = nanoid!();
            return Value::from_float(appfn(res.cast_float().unwrap()));
        }
        return Value::new();
    }
}
