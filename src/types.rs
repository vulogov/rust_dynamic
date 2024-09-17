use std::collections::hash_map::HashMap;
use crate::value::Value;
use serde::{Deserialize, Serialize};
use crate::error::{BundError};
use crate::metric::Metric;

pub type AppFn0  = fn();
pub type AppFn  = fn(Value) -> Value;
pub type AppFn2 = fn(Value, Value) -> Value;
pub type MaybeFn = fn(&Value) -> bool;
pub type MaybeFn2 = fn(&Value, &Value) -> bool;
pub type FloatFn = fn(f64) -> f64;

pub const NONE: u16         = 0;
pub const BOOL: u16         = 1;
pub const INTEGER: u16      = 2;
pub const FLOAT: u16        = 3;
pub const STRING: u16       = 4;
pub const LITERAL: u16      = 5;
pub const CALL: u16         = 6;
pub const PTR: u16          = 7;
pub const BIN: u16          = 8;
pub const LIST: u16         = 9;
pub const PAIR: u16         = 10;
pub const MAP: u16          = 11;
pub const ENVELOPE: u16     = 12;
pub const TIME: u16         = 13;
pub const CINTEGER: u16     = 14;
pub const CFLOAT: u16       = 15;
pub const METRICS: u16      = 16;
pub const LAMBDA: u16       = 17;
pub const QUEUE: u16        = 18;
pub const FIFO: u16         = 19;
pub const OPERATOR: u16     = 20;
pub const CONTEXT: u16      = 21;
pub const RESULT: u16       = 92;
pub const EXIT: u16         = 93;
pub const ASSOCIATION: u16  = 94;
pub const CONFIG: u16       = 95;
pub const INFO: u16         = 96;
pub const NODATA: u16       = 97;
pub const ERROR: u16        = 98;
pub const TOKEN: u16        = 99;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Operator {
    pub opcode:     i32,
    pub opvalue1:    Vec<u8>,
    pub opvalue2:    Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Val {
    Null,
    Exit,
    Token(String),
    Error(BundError),
    Bool(bool),
    I64(i64),
    F64(f64),
    List(Vec<Value>),
    Lambda(Vec<Value>),
    Queue(Vec<Value>),
    Map(HashMap<String, Value>),
    String(String),
    Binary(Vec<u8>),
    Time(u128),
    Metrics(Vec<Metric>),
    Operator(Operator),
}
