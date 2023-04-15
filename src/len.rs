use crate::value::Value;
use crate::types::*;

impl Value {
    pub fn len(&self) -> usize {
        match self.dt {
            LIST | RESULT => {
                match &self.data {
                    Val::List(v) => return v.len(),
                    _ => return 0,
                }
            }
            BIN | ENVELOPE => {
                match &self.data {
                    Val::Binary(v) => return v.len(),
                    _ => return 0,
                }
            }
            MAP => {
                match &self.data {
                    Val::Map(v) => return v.len(),
                    _ => return 0,
                }
            }
            METRICS => {
                match &self.data {
                    Val::Metrics(v) => return v.len(),
                    _ => return 0,
                }
            }
            NODATA => return 0,
            _ => return 1,
        }
    }
}
