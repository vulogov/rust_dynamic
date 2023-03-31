use crate::value::Value;
use crate::types::*;

impl Value {
    pub fn car(&self) -> Option<Self> {
        match self.dt {
            LIST => {
                match &self.data {
                    Val::List(l_val) => {
                        if l_val.len() > 0 {
                            return Some(l_val[0].clone());
                        } else {
                            return None;
                        }
                    }
                    _ => None,
                }
            }
            _ => Some(self.clone()),
        }
    }
    pub fn cdr(&self) -> Option<Self> {
        match self.dt {
            LIST => {
                match &self.data {
                    Val::List(l_val) => {
                        if l_val.len() > 0 {
                            return Some(Value::from_list(l_val[1..].to_vec()));
                        } else {
                            return None;
                        }
                    }
                    _ => None,
                }
            }
            _ => Some(self.clone()),
        }
    }
    pub fn last(&self) -> Option<Self> {
        match self.dt {
            LIST => {
                match &self.data {
                    Val::List(l_val) => {
                        if l_val.len() > 0 {
                            return Some(l_val[l_val.len()].clone());
                        } else {
                            return None;
                        }
                    }
                    _ => None,
                }
            }
            _ => Some(self.clone()),
        }
    }
}
