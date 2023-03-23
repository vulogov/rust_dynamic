use crate::value::Value;
use std::f64::consts::{PI,E};

impl Value {
    pub fn pi() -> Self {
        Value::from_float(PI as f64)
    }
    pub fn e() -> Self {
        Value::from_float(E as f64)
    }
    pub fn nan() -> Self {
        Value::from_float(f64::NAN)
    }
    pub fn inf() -> Self {
        Value::from_float(f64::INFINITY)
    }
    pub fn negative_inf() -> Self {
        Value::from_float(f64::NEG_INFINITY)
    }
}
