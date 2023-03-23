use crate::value::Value;
use crate::types::*;
use bincode2;
use nanoid::nanoid;

impl Value {
    pub fn to_binary(&self) -> Result<Vec<u8>, bincode2::Error> {
        bincode2::serialize(self)
    }
    pub fn from_binary(data: Vec<u8>) -> Result<Value, bincode2::Error> {
        bincode2::deserialize::<Value>(&data)
    }
    pub fn wrap(&self) -> Result<Value, bincode2::Error> {
        match self.to_binary() {
            Ok(res) => {
                return Result::Ok(Value::from_bin(res));
            }
            Err(err) => return Err(err),
        }
    }
    pub fn unwrap(&self) -> Result<Value, Box<dyn std::error::Error>> {
        if self.dt != BIN {
            return Err("You requested to unwrap a non-binary object".into());
        }
        match &self.data {
            Val::Binary(data) => {
                match Value::from_binary(data.clone()) {
                    Ok(mut res) => {
                        res.id = nanoid!();
                        return Result::Ok(res);
                    }
                    Err(err) => return Err(err),
                }
            }
            _ => Err(format!("Unwrappable object {}", self.id).into()),
        }
    }
}
