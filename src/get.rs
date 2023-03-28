use crate::value::Value;
use crate::types::*;

impl Value {
    pub fn get(&self, key: String) -> Result<Self, Box<dyn std::error::Error>> {
        match self.dt {
            MAP | INFO | CONFIG | ASSOCIATION => {
                match &self.data {
                    Val::Map(v) => {
                        match v.get(&key) {
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
