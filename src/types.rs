use serde::{Deserialize, Serialize};
use crate::error::{BundError};

pub const NONE: u16     = 0;
pub const BOOL: u16     = 1;
pub const INTEGER: u16  = 2;
pub const FLOAT: u16    = 3;
pub const STRING: u16   = 4;
pub const LITERAL: u16  = 5;
pub const CALL: u16     = 6;
pub const PTR: u16      = 7;
pub const BIN: u16      = 8;
pub const LIST: u16     = 9;
pub const NODATA: u16   = 97;
pub const ERROR: u16    = 98;
pub const TOKEN: u16    = 99;



#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Val {
    Null,
    Token,
    Error(BundError),
    Bool(bool),
    I64(i64),
    F64(f64),
    List(Vec<Val>),
    String(String),
    Binary(Vec<u8>),
}
