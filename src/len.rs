use crate::value::Value;
use crate::types::*;

impl Value {
    pub fn len(&self) -> usize {
        match self.dt {
            JSON => {
                match self.cast_json() {
                    Ok(j_value) => {
                        if j_value.is_array() {
                            return j_value.as_array().unwrap().len();
                        } else {
                            return 1;
                        }
                    }
                    Err(_) => {
                        return 0;
                    }
                }
            }
            STRING | TEXTBUFFER => {
                match &self.data {
                    Val::String(v) => return v.len(),
                    _ => return 0,
                }
            }
            LIST | RESULT => {
                match &self.data {
                    Val::List(v) => return v.len(),
                    _ => return 0,
                }
            }
            MATRIX => {
                match &self.data {
                    Val::Matrix(v) => {
                        let mut c: usize = 0;
                        for r in v.iter() {
                            c = c + r.len();
                        }
                        return c;
                    }
                    _ => return 0,
                }
            }
            QUEUE | FIFO => {
                match &self.data {
                    Val::Queue(v) => return v.len(),
                    _ => return 0,
                }
            }
            LAMBDA => {
                match &self.data {
                    Val::Lambda(v) => return v.len(),
                    _ => return 0,
                }
            }
            BIN | ENVELOPE => {
                match &self.data {
                    Val::Binary(v) => return v.len(),
                    _ => return 0,
                }
            }
            MAP | CURRY | CONDITIONAL | CLASS | OBJECT => {
                match &self.data {
                    Val::Map(v) => return v.len(),
                    _ => return 0,
                }
            }
            VALUEMAP => {
                match &self.data {
                    Val::ValueMap(v) => return v.len(),
                    _ => return 0,
                }
            }
            MESSAGE => {
                let _ = match self.get("payload") {
                    Ok(res) => {
                        return res.len();
                    }
                    Err(_) => return 0,
                };
            }
            METRICS => {
                match &self.data {
                    Val::Metrics(v) => return v.len(),
                    _ => return 0,
                }
            }
            NODATA => return 0,
            _ => {
                let str_val = match self.conv(STRING) {
                    Ok(str_val) => str_val,
                    Err(_) => {
                        return 1;
                    }
                };
                return str_val.len();
            }
        }
    }
}
