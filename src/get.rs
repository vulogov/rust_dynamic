use crate::value::Value;
use crate::types::*;

impl Value {
    pub fn get<N: AsRef<str> + std::fmt::Display>(&self, key: N) -> Result<Self, Box<dyn std::error::Error>> {
        match self.dt {
            MAP | INFO | CONFIG | ASSOCIATION | CURRY | MESSAGE | CONDITIONAL => {
                match &self.data {
                    Val::Map(v) => {
                        match v.get(&key.as_ref().to_string()) {
                            Some(d) => return Result::Ok(d.clone()),
                            None => return Err(format!("Key not found: {}", &key).into()),
                        }
                    }
                    _ => Err("Data element is corrupted".into()),
                }
            }
            _ => {
                Result::Ok(self.clone())
            }
        }
    }
}
