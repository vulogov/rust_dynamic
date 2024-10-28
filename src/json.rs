use crate::value::Value;
use crate::types::*;
use easy_error::{Error, bail};
use serde_json::{from_str as json_from_string};

impl Value {
    pub fn to_json(&self) -> Result<String, Error> {
        match self.conv(STRING) {
            Ok(j_value_str) => {
                match j_value_str.cast_string() {
                    Ok(str_data) => {
                        return Ok(str_data);
                    }
                    Err(err) => {
                        bail!("{}", err);
                    }
                }
            }
            Err(err) => {
                bail!("{}", err);
            }
        }
    }
    pub fn from_json(data: String) -> Result<Value, Error> {
        match json_from_string(&data) {
            Ok(j_value) => {
                let json_value = Value::json(j_value);
                match json_value.cast_json_to_value() {
                    Ok(value) => {
                        return Ok(value);
                    }
                    Err(err) => {
                        bail!("{}", err);
                    }
                }
            }
            Err(err) => {
                bail!("{}", err);
            }
        }
    }
}
