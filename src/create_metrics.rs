use crate::value::{Value, timestamp_ms};
use crate::metric::Metric;
use nanoid::nanoid;
use crate::types::*;

impl Value {
    pub fn metrics_n(n: usize) -> Self {
        let mut data: Vec<Metric> = Vec::new();
        for _ in 0..n {
            data.push(Metric::new(0.0 as f64));
        }
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   METRICS,
            q:    100.0,
            data: Val::Metrics(data),
            attr: Vec::new(),
            curr: -1,
        }
    }
    pub fn metrics() -> Self {
        Value::metrics_n(128)
    }
    pub fn from_metrics(value: Vec<Metric>) -> Self {
        Self {
            id:   nanoid!(),
            stamp:  timestamp_ms(),
            dt:   METRICS,
            q:    100.0,
            data: Val::Metrics(value),
            attr: Vec::new(),
            curr: -1,
        }
    }
}
