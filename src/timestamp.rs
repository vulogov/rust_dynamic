use crate::value::{Value, timestamp_ns};
use crate::types::*;

impl Value {
    pub fn get_timestamp(&self) -> f64 {
        self.stamp
    }
    pub fn timestamp_diff(&self, other: Self) -> f64 {
        self.stamp - other.get_timestamp()
    }
    pub fn elapsed(&self) -> Result<f64, Box<dyn std::error::Error>> {
        if self.dt != TIME {
            return Err("Incorrect type for the method, TIME required".into());
        }
        match self.data {
            Val::F64(f_val) => {
                return Result::Ok(timestamp_ns() - f_val);
            }
            _ => Err("Value of type TIME is corrupted".into()),
        }
    }
    pub fn elapsed_value(&self) -> Result<Self, Box<dyn std::error::Error>> {
        if self.dt != TIME {
            return Err("Incorrect type for the method, TIME required".into());
        }
        match self.data {
            Val::F64(f_val) => {
                return Result::Ok(Value::from_float(timestamp_ns() - f_val));
            }
            _ => Err("Value of type TIME is corrupted".into()),
        }
    }
}
