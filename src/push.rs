use crate::value::{Value, timestamp_ms};
use crate::metric::Metric;
use crate::types::*;
use nanoid::nanoid;

impl Value {
    pub fn push(&mut self, value: Value) -> Self {
        match self.dt {
            JSON => {
                match &self.data {
                    Val::Json(v) => {
                        if ! v.is_array() {
                            return self.clone();
                        }
                        match value.cast_value_to_json() {
                            Ok(j_value) => {
                                match self.cast_json() {
                                    Ok(mut j_arr_value) => {
                                        let j_array = j_arr_value.as_array_mut().unwrap();
                                        j_array.push(j_value);
                                        return Value::json(serde_json::to_value(j_array).unwrap());
                                    }
                                    Err(_) => {
                                        return self.clone();
                                    }
                                }
                            }
                            Err(_) => {
                                return self.clone();
                            }
                        }
                    }
                    _ => {}
                }
                return self.clone();
            }
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
            MATRIX => {
                let mut data: Vec<Vec<Value>> = Vec::new();
                match &self.data {
                    Val::Matrix(v) => {
                        for i in v {
                            data.push(i.clone());
                        }
                        if value.type_of() == LIST {
                            let list_data = match value.cast_list() {
                                Ok(list_data) => list_data,
                                Err(_) => {
                                    return self.clone();
                                }
                            };
                            data.push(list_data);
                        }
                    }
                    _ => {},
                }
                return Value::from_matrix(data);
            }
            TEXTBUFFER => {
                match self.cast_string() {
                    Ok(str_val) => {
                        let mut res = Value::text_buffer(str_val);
                        res = res + value;
                        return res;
                    }
                    Err(_) => {
                        return self.clone();
                    }
                }
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
