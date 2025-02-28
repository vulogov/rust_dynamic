use std::collections::hash_map::HashMap;
use crate::value::Value;
use crate::types::*;

impl Value {
    pub fn fmap(&mut self, appfn: AppFn) -> Self {
        match self.dt {
            JSON => {
                let mut data: Vec<Value> = Vec::new();
                match &self.data {
                    Val::Json(v) => {
                        if v.is_array() {
                            for n in v.as_array().unwrap() {
                                match Value::json(n.clone()).cast_json_to_value() {
                                    Ok(value) => {
                                        data.push(appfn(value));
                                    }
                                    Err(_) => {
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    _ => {},
                }
                return Value::from_list(data);
            }
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
            MAP | INFO | ASSOCIATION | CONFIG | CONDITIONAL | CLASS | OBJECT => {
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
            VALUEMAP => {
                let mut data: HashMap<Value, Value> = HashMap::new();
                let mut res = self.dup().unwrap().regen_id();
                match &self.data {
                    Val::ValueMap(m_data) => {
                        for (k,v) in m_data {
                            data.insert(k.clone(), appfn(v.clone()));
                        }
                    }
                    _ => {},
                }
                res.data = Val::ValueMap(data);
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
