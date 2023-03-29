use unix_ts::Timestamp;
use chrono::{DateTime, Utc};
use crate::value::{Value, timestamp_ns};
use crate::types::*;

impl Value {
    pub fn get_timestamp(&self) -> f64 {
        self.stamp
    }
    pub fn get_timestamp_as_datetime(&self) -> DateTime<Utc> {
        let ts = Timestamp::from_millis(self.stamp as i128);
        ts.to_utc_datetime()
    }
    pub fn timestamp_diff(&self, other: Self) -> f64 {
        self.stamp - other.get_timestamp()
    }
    pub fn elapsed(&self) -> Result<u128, Box<dyn std::error::Error>> {
        if self.dt != TIME {
            return Err("Incorrect type for the method, TIME required".into());
        }
        match self.data {
            Val::Time(u_val) => {
                return Result::Ok(timestamp_ns() - u_val);
            }
            _ => Err("Value of type TIME is corrupted".into()),
        }
    }
    pub fn elapsed_value(&self) -> Result<Self, Box<dyn std::error::Error>> {
        if self.dt != TIME {
            return Err("Incorrect type for the method, TIME required".into());
        }
        match self.data {
            Val::Time(u_val) => {
                return Result::Ok(Value::from_timestamp(timestamp_ns() - u_val));
            }
            _ => Err("Value of type TIME is corrupted".into()),
        }
    }
    pub fn get_time_as_datetime(&self) -> Result<DateTime<Utc>, Box<dyn std::error::Error>> {
        if self.dt != TIME {
            return Err("Value must be of type TIME".into());
        }
        match self.data {
            Val::Time(u_val) => {
                let ts = Timestamp::from_nanos(u_val as i128);
                return Result::Ok(ts.to_utc_datetime());
            }
            _ => Err("Value of type TIME is corrupted".into()),
        }
    }
}
