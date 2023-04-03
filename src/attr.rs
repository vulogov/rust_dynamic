use nanoid::nanoid;
use crate::value::Value;

impl Value {
    pub fn attr(&mut self, value: Vec<Value>) -> Self {
        let mut res = self.clone();
        res.id = nanoid!();
        res.attr = value;
        return res;
    }
    pub fn attr_merge(&mut self, value: Vec<Value>) -> Self {
        let mut v   = value.clone();
        let mut res = self.dup().expect("dup expected").regen_id();
        v.extend(res.attr);
        res.attr = v;
        return res;
    }
    pub fn attr_add(&mut self, value: Value) -> Self {
        let mut res = self.dup().expect("dup expected").regen_id();
        res.attr.push(value);
        return res;
    }
    pub fn attr_len(&self) -> usize {
        self.attr.len()
    }
}
