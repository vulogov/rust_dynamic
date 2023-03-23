use nanoid::nanoid;
use crate::value::Value;

impl Value {
    pub fn attr(&mut self, value: Vec<Value>) -> Self {
        let mut res = self.clone();
        res.id = nanoid!();
        res.attr = value;
        return res;
    }
    pub fn attr_add(&mut self, value: Value) -> Self {
        let mut res = self.clone();
        res.id = nanoid!();
        res.attr.push(value);
        return res;
    }
    pub fn attr_len(&self) -> usize {
        self.attr.len()
    }
}
