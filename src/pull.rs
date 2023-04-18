use crate::value::{Value};
use crate::types::*;

impl Value {
    pub fn pull(&self) -> Self {
        match self.dt {
            QUEUE => {
                let mut data: Vec<Value> = Vec::new();
                match &self.data {
                    Val::Queue(v) => {
                        if v.len() > 0 {
                            for i in &v[1..] {
                                data.push(i.clone());
                            }
                        }
                    }
                    _ => {},
                }
                return Value::to_queue(data);
            }
            FIFO => {
                let mut data: Vec<Value> = Vec::new();
                match &self.data {
                    Val::Queue(v) => {
                        if v.len() > 0 {
                            for i in &v[..self.len()-1] {
                                data.push(i.clone());
                            }
                        }
                    }
                    _ => {},
                }
                return Value::to_fifo(data);
            }
            LIST | RESULT => {
                let mut data: Vec<Value> = Vec::new();
                match &self.data {
                    Val::List(v) => {
                        for i in &v[..self.len()-1] {
                            data.push(i.clone());
                        }
                    }
                    _ => {},
                }
                if self.dt == LIST {
                    return Value::from_list(data);
                } else {
                    return Value::to_result(data);
                }
            }
            _ => {
                let mut res = self.dup().unwrap();
                res.regen_id();
                return res;
            }
        }
    }
}
