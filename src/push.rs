use crate::value::{Value, timestamp_ms};
use crate::metric::Metric;
use crate::types::*;
use nanoid::nanoid;

impl Value {
    pub fn push(&mut self, value: Value) -> Self {
        match self.dt {
            LIST | RESULT => {
                let mut data: Vec<Value> = Vec::new();
                match &self.data {
                    Val::List(v) => {
                        for i in v {
                            data.push(i.clone());
                        }
                        data.push(value.clone());
                    }
                    _ => {},
                }
                return Value::from_list(data);
            }
            QUEUE | FIFO => {
                let mut data: Vec<Value> = Vec::new();
                match &self.data {
                    Val::Queue(v) => {
                        for i in v {
                            data.push(i.clone());
                        }
                        data.push(value.clone());
                    }
                    _ => {},
                }
                if self.dt == QUEUE {
                    return Value::to_queue(data);
                } else {
                    return Value::to_fifo(data);
                }
            }
            LAMBDA => {
                let mut data: Vec<Value> = Vec::new();
                match &self.data {
                    Val::Lambda(v) => {
                        for i in v {
                            data.push(i.clone());
                        }
                        data.push(value.clone());
                    }
                    _ => {},
                }
                return Value::to_lambda(data);
            }
            METRICS => {
                if value.dt != FLOAT {
                    return self.clone();
                }
                match &self.data {
                    Val::Metrics(v) => {
                        let mut m_data: Vec<Metric> = Vec::new();
                        for i in v {
                            m_data.push(i.clone());
                        }
                        m_data.push(Metric::new(value.cast_float().unwrap()));
                        m_data.remove(0);
                        return Value::from_metrics(m_data);
                    }
                    _ => return self.clone(),
                }
            }
            _ => {
                let mut res = value.clone();
                res.q = self.q;
                res.id = nanoid!();
                res.stamp = timestamp_ms();
                return res;
            }
        }
    }
}
