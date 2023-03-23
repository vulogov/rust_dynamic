use crate::value::Value;
use crate::types::*;

impl Value {
    pub fn len(&self) -> usize {
        match self.dt {
            LIST => {
                match &self.data {
                    Val::List(v) => return v.len(),
                    _ => return 0,
                }
            }
            NODATA => return 0,
            _ => return 1,
        }
    }
}
