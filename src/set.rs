use crate::value::Value;
use crate::types::*;
use nanoid::nanoid;

impl Value {
    pub fn set(&mut self, key: String, value: Value) -> Self {
        match self.dt {
            LIST => {
                return Value::from_list(vec![value]);
            }
            MAP | INFO | CONFIG | ASSOCIATION => {
                let mut res = self.dup().unwrap();
                res.id = nanoid!();
                match &res.data {
                    Val::Map(v) => {
                        let mut m = v.clone();
                        m.insert(key, value);
                        return Value::from_dict(m);
                    }
                    _ => {}
                }
                return res;
            }
            _ => {
                let mut res = value.clone();
                res.id = nanoid!();
                res.q = self.q;
                return res;
            }
        }
    }
}
