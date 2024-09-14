use crate::value::{Value};
use crate::metric::Metric;
use crate::types::*;
use easy_error::{Error, bail};

impl Value {
    pub fn push_inplace(&mut self, value: Value) -> Result<&mut Value, Error> {
        match self.dt {
            LIST | RESULT => {
                match &mut self.data {
                    Val::List(ref mut v) => {
                        let _ = v.push(value.clone());
                    }
                    _ => {
                        bail!("Invalid List format for push() operation");
                    }
                }
                return Ok(self);
            }
            QUEUE | FIFO => {
                match &mut self.data {
                    Val::Queue(ref mut v) => {
                        let _ = v.push(value.clone());
                    }
                    _ => {
                        bail!("Invalid Queue/Fifo format for push() operation");
                    }
                }
                return Ok(self)
            }
            LAMBDA => {
                match &mut self.data {
                    Val::Lambda(ref mut v) => {
                        let _ = v.push(value.clone());
                    }
                    _ => {
                        bail!("Invalid Lambda format for push() operation");
                    }
                }
                return Ok(self);
            }
            METRICS => {
                if value.dt != FLOAT {
                    return Ok(self);
                }
                match &mut self.data {
                    Val::Metrics(ref mut v) => {
                        let _ = &v.push(Metric::new(value.cast_float().unwrap()));
                        let _ = &v.remove(0);
                        return Ok(self);
                    }
                    _ => {
                        bail!("Invalid Metric format for push() operation");
                    }
                }
            }
            _ => {
                bail!("This Value do not support push() operation");
            }
        }
    }
}
