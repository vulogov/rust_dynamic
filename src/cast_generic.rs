use serde_json::value;
use serde_json::{json, Map};
use crate::value::Value;
use crate::types::*;

impl Value {
    pub fn as_json_value(&self) -> value::Value {
        match self.dt {
            INTEGER =>  json!(self.cast_int().unwrap()),
            FLOAT =>    json!(self.cast_float().unwrap()),
            STRING =>   json!(self.cast_string().unwrap()),
            BOOL =>   json!(self.cast_bool().unwrap()),
            LIST => {
                let mut res: Vec<value::Value> = Vec::new();
                match &self.data {
                    Val::List(v) => {
                        for i in v {
                            res.push(i.as_json_value());
                        }
                    }
                    _ => {},
                }
                return json!(res);
            }
            MAP => {
                let mut res: Map<String, value::Value> = Map::new();
                match &self.data {
                    Val::Map(v) => {
                        for (k, val) in v {
                            res.insert(k.to_string(), val.as_json_value());
                        }
                    }
                    _ => {},
                }
                return json!(res);
            }
            _ => json!(null),
        }
    }
}
