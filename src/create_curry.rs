use crate::value::{Value, timestamp_ms};
use std::collections::HashMap;
use nanoid::nanoid;
use crate::types::*;

impl Value {
    pub fn curry<N: AsRef<str>>(name: N, curry_data: Vec<Value>) -> Self {
        let mut data: HashMap<String, Value> = HashMap::new();
        data.insert("name".to_string(), Value::from_string(name));
        data.insert("ptr".to_string(), Value::none());
        data.insert("lambda".to_string(), Value::none());
        data.insert("data".to_string(), Value::from_list(curry_data));
        Self {
            id:     nanoid!(),
            stamp:  timestamp_ms(),
            dt:     CURRY,
            q:      100.0,
            data:   Val::Map(data),
            attr:   Vec::new(),
            curr:   -1,
            tags:   HashMap::new(),
        }
    }

    pub fn ptr_curry<N: AsRef<str> + ToString>(name: N, ptr: N, curry_data: Vec<Value>) -> Self {
        let mut data: HashMap<String, Value> = HashMap::new();
        data.insert("name".to_string(), Value::from_string(name));
        data.insert("ptr".to_string(), Value::ptr(ptr.to_string(), Vec::new()));
        data.insert("lambda".to_string(), Value::none());
        data.insert("data".to_string(), Value::from_list(curry_data));
        Self {
            id:     nanoid!(),
            stamp:  timestamp_ms(),
            dt:     CURRY,
            q:      100.0,
            data:   Val::Map(data),
            attr:   Vec::new(),
            curr:   -1,
            tags:   HashMap::new(),
        }
    }

    pub fn lambda_curry<N: AsRef<str>>(name: N, lambda: Vec<Value>, curry_data: Vec<Value>) -> Self {
        let mut data: HashMap<String, Value> = HashMap::new();
        data.insert("name".to_string(), Value::from_string(name));
        data.insert("ptr".to_string(), Value::none());
        data.insert("lambda".to_string(), Value::to_lambda(lambda));
        data.insert("data".to_string(), Value::from_list(curry_data));
        Self {
            id:     nanoid!(),
            stamp:  timestamp_ms(),
            dt:     CURRY,
            q:      100.0,
            data:   Val::Map(data),
            attr:   Vec::new(),
            curr:   -1,
            tags:   HashMap::new(),
        }
    }

}
