use nanoid::nanoid;
use num::complex::Complex;
use crate::value::{Value, timestamp_ms};
use crate::types::*;

impl Value {
    pub fn from_complex_int(value: Complex<i64>) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   CINTEGER,
            q:    100.0,
            data: Val::List(vec![Value::from_int(value.re as i64), Value::from_int(value.im as i64)]),
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn from_complex_float(value: Complex<f64>) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   CFLOAT,
            q:    100.0,
            data: Val::List(vec![Value::from_float(value.re as f64), Value::from_float(value.im as f64)]),
            attr: Vec::new(),
            curr: -1,
        }
    }
}
