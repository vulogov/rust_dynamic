use crate::value::Value;
use crate::types::*;

impl Value {
    pub fn has_key<N: AsRef<str> + std::fmt::Display>(&self, key: N) -> Result<Self, Box<dyn std::error::Error>> {
        match self.dt {
            MAP | INFO | CONFIG | ASSOCIATION | CURRY | CONDITIONAL => {
                match &self.data {
                    Val::Map(v) => {
                        if v.contains_key(&key.as_ref().to_string()) {
                            return Ok(Value::make_true());
                        } else {
                            return Ok(Value::make_false());
                        }
                    }
                    _ => Err("Data element is corrupted".into()),
                }
            }
            _ => {
                Result::Ok(Value::make_false())
            }
        }
    }
    pub fn has_key_raw<N: AsRef<str> + std::fmt::Display>(&self, key: N) -> Result<bool, Box<dyn std::error::Error>> {
        match self.dt {
            MAP | INFO | CONFIG | ASSOCIATION | CURRY | CONDITIONAL => {
                match &self.data {
                    Val::Map(v) => {
                        if v.contains_key(&key.as_ref().to_string()) {
                            return Ok(true);
                        } else {
                            return Ok(false);
                        }
                    }
                    _ => Err("Data element is corrupted".into()),
                }
            }
            _ => {
                Result::Ok(false)
            }
        }
    }
}
