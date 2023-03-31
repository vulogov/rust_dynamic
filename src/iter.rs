use crate::value::Value;
use crate::types::*;

impl Iterator for Value {
    type Item = Value;
    fn next(&mut self) -> Option<Value> {
         if self.curr == -1 && self.dt == LIST {
            match &self.data {
                Val::List(v) => {
                    if v.len() > 0 {
                        self.curr = 1;
                        return Some(v[0].clone());
                    } else {
                        return None;
                    }
                }
                _ => return None,
            }
        } else if self.curr >= 0 && self.dt == LIST {
            match &self.data {
                Val::List(v) => {
                    if v.len() > self.curr as usize {
                        let ret = v[self.curr as usize].clone();
                        self.curr += 1;
                        return Some(ret);
                    } else {
                        self.curr = -1;
                        return None;
                    }
                }
                _ => return None,
            }
        } else if self.curr == -1 && self.dt == METRICS {
            match &self.data {
                Val::Metrics(v) => {
                    if v.len() > 0 {
                        self.curr = 1;
                        let m = &v[0];
                        let mut res = Value::dict();
                        res = res.set("value", Value::from_float(m.data as f64));
                        res = res.set("ts", Value::from_stamp(m.stamp as u128));
                        return Some(res);
                    } else {
                        return None;
                    }
                }
                _ => return None,
            }
        } else if self.curr >= 0 && self.dt == METRICS {
            match &self.data {
                Val::Metrics(v) => {
                    if v.len() > self.curr as usize {
                        let m = &v[self.curr as usize];
                        let mut res = Value::dict();
                        res = res.set("value", Value::from_float(m.data as f64));
                        res = res.set("ts", Value::from_stamp(m.stamp as u128));
                        self.curr += 1;
                        return Some(res);
                    } else {
                        self.curr = -1;
                        return None;
                    }
                }
                _ => return None,
            }
        } else if self.curr == -1 {
            self.curr = 0;
            return Some(self.clone());
        } else if self.curr == 0 {
            self.curr = -1;
            return None;
        } else {
            return None;
        }
    }
}
