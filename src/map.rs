use std::collections::hash_map::HashMap;
use crate::value::Value;
use crate::types::*;

impl Value {
    pub fn fmap(&mut self, appfn: AppFn) -> Self {
        match self.dt {
            LIST | RESULT => {
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
            LAMBDA => {
                let mut data: Vec<Value> = Vec::new();
                match &self.data {
                    Val::Lambda(v) => {
                        for i in v {
                            data.push(appfn(i.clone()));
                        }
                    }
                    _ => {},
                }
                return Value::to_lambda(data);
            }
            MAP | INFO | ASSOCIATION | CONFIG => {
                let mut data: HashMap<String, Value> = HashMap::new();
                let mut res = self.dup().unwrap().regen_id();
                match &self.data {
                    Val::Map(m_data) => {
                        for (k,v) in m_data {
                            data.insert(k.clone(), appfn(v.clone()));
                        }
                    }
                    _ => {},
                }
                res.data = Val::Map(data);
                return res;
            }
            NONE => {
                self.clone()
            }
            _ => {
                let res = self.dup().unwrap().regen_id();
                return appfn(res);
            }
        }
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
            return Value::from_float(appfn(self.cast_float().unwrap()));
        }
        return Value::new();
    }
}
