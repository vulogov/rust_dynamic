use std::fmt;
use crate::value::{Value};
use crate::types::*;

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.conv(STRING) {
            Ok(val) => write!(f, "{}", val.cast_string().unwrap()),
            Err(_) => write!(f, "Value({})={:?}", self.dt, self),
        }
    }
}
