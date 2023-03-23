use crate::value::Value;
use bincode2;

impl Value {
    pub fn to_binary(&self) -> Result<Vec<u8>, bincode2::Error> {
        bincode2::serialize(self)
    }
    pub fn from_binary(data: Vec<u8>) -> Result<Value, bincode2::Error> {
        bincode2::deserialize::<Value>(&data)
    }
}
