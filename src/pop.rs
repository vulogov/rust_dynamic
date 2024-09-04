use crate::value::{Value};
use crate::types::*;

impl Value {
    pub fn pop(&mut self) -> Option<Value> {
        match self.dt {
            LIST | RESULT => {
                match &mut self.data {
                    Val::List(ref mut v) => {
                        return v.pop().clone();
                    }
                    _ => return None,
                }
            }
            FIFO => {
                match &mut self.data {
                    Val::Queue(ref mut v) => {
                        return v.pop().clone();
                    }
                    _ => return None,
                }
            }
            _ => {
                return None;
            }
        }
    }
}
