use crate::value::{Value, timestamp_ms};
use crate::types::*;
use nanoid::nanoid;

impl Value {
    pub fn set<N: AsRef<str> + std::fmt::Display>(&mut self, key: N, value: Value) -> Self {
        match self.dt {
            LIST | RESULT => {
                return Value::from_list(vec![value]);
            }
            LAMBDA => {
                return Value::to_lambda(vec![value]);
            }
            MAP | INFO | CONFIG | ASSOCIATION | CURRY => {
                let mut res = self.dup().unwrap();
                res.id = nanoid!();
                match &res.data {
                    Val::Map(v) => {
                        let mut m = v.clone();
                        m.insert(key.as_ref().to_string().trim().to_string(), value);
                        return Value::from_dict(m);
                    }
                    _ => {}
                }
                return res;
            }
            _ => {
                let mut res = value.clone();
                res.id = nanoid!();
                res.stamp = timestamp_ms();
                res.q = self.q;
                return res;
            }
        }
    }

    pub fn set_key_raw<N: AsRef<str> + std::fmt::Display>(&mut self, key: N, value: Value) -> Self {
        match self.dt {
            LIST | RESULT => {
                return Value::from_list(vec![value]);
            }
            LAMBDA => {
                return Value::to_lambda(vec![value]);
            }
            MAP | INFO | CONFIG | ASSOCIATION | CURRY => {
                let mut res = self.dup().unwrap();
                res.id = nanoid!();
                match &res.data {
                    Val::Map(v) => {
                        let mut m = v.clone();
                        m.insert(key.as_ref().to_string(), value);
                        return Value::from_dict(m);
                    }
                    _ => {}
                }
                return res;
            }
            _ => {
                let mut res = value.clone();
                res.id = nanoid!();
                res.stamp = timestamp_ms();
                res.q = self.q;
                return res;
            }
        }
    }
}
