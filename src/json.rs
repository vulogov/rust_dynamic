use crate::value::Value;
use serde_json::{Error, to_string as json_to_string, from_str as json_from_string};

impl Value {
    pub fn to_json(&self) -> Result<String, Error> {
        json_to_string(self)
    }
    pub fn from_json(data: String) -> Result<Value, Error> {
        json_from_string::<Value>(&data)
    }
}
