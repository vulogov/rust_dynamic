use crate::value::Value;
use crate::types::*;

impl Value {
    pub fn head(&self, n: i64) -> Option<Self> {
        match self.dt {
            LIST => {
                match self.cast_list() {
                    Ok(raw_list) => {
                        let mut res = Value::list();
                        let mut c: usize = 0;
                        for v in raw_list {
                            if c >= n as usize {
                                break;
                            }
                            res = res.push(v);
                            c = c + 1;
                        }
                        return Some(res);
                    }
                    Err(_) => {
                        return None;
                    }
                }
            }
            _ => {
                return Some(self.clone());
            }
        }
    }
    pub fn tail(&self, n: i64) -> Option<Self> {
        match self.dt {
            LIST => {
                match self.cast_list() {
                    Ok(raw_list) => {
                        let mut res = Value::list();
                        let mut c: usize = raw_list.len();
                        for v in raw_list.into_iter().rev().clone().into_iter() {
                            if c <= n as usize  || c == 0 {
                                break;
                            }
                            res = res.push(v);
                            c = c - 1;
                        }
                        return Some(res);
                    }
                    Err(_) => {
                        return None;
                    }
                }
            }
            _ => {
                return Some(self.clone());
            }
        }
    }
    pub fn at(&self, n: i64) -> Option<Self> {
        match self.dt {
            LIST => {
                match self.cast_list() {
                    Ok(raw_list) => {
                        if n as usize > raw_list.len() || n < 0 {
                            return None;
                        }
                        return Some(raw_list[n as usize].clone());
                    }
                    Err(_) => {
                        return None;
                    }
                }
            }
            _ => {
                return Some(self.clone());
            }
        }
    }
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
            MATRIX => {
                match &self.data {
                    Val::Matrix(m_val) => {
                        if m_val.len() > 0 {
                            return Some(Value::from_list(m_val[0].clone()));
                        } else {
                            return None;
                        }
                    }
                    _ => None,
                }
            }
            QUEUE => {
                match &self.data {
                    Val::Queue(_) => {
                        match self.cast_queue() {
                            Ok(val) => return Some(val),
                            Err(_) => return None,
                        }
                    }
                    _ => None,
                }
            }
            FIFO => {
                match &self.data {
                    Val::Queue(_) => {
                        match self.cast_fifo() {
                            Ok(val) => return Some(val),
                            Err(_) => return None,
                        }
                    }
                    _ => None,
                }
            }
            METRICS => {
                match &self.data {
                    Val::Metrics(m_val) => {
                        if m_val.len() > 0 {
                            let m = &m_val[0];
                            let mut res = Value::dict();
                            res = res.set("value", Value::from_float(m.data as f64));
                            res = res.set("ts", Value::from_stamp(m.stamp as u128));
                            return Some(res);
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
            MATRIX => {
                match &self.data {
                    Val::Matrix(m_val) => {
                        if m_val.len() > 0 {
                            return Some(Value::from_matrix(m_val[1..].to_vec()));
                        } else {
                            return None;
                        }
                    }
                    _ => None,
                }
            }
            QUEUE | FIFO => return Some(self.pull()),
            METRICS => {
                match &self.data {
                    Val::Metrics(m_val) => {
                        if m_val.len() > 0 {
                            let m = &m_val[1..].to_vec();
                            return Some(Value::from_metrics(m.clone()));
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
            METRICS => {
                match &self.data {
                    Val::Metrics(m_val) => {
                        if m_val.len() > 0 {
                            let m = &m_val[m_val.len()];
                            let mut res = Value::dict();
                            res = res.set("value", Value::from_float(m.data as f64));
                            res = res.set("ts", Value::from_stamp(m.stamp as u128));
                            return Some(res);
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
