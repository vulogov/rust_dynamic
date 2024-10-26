use crate::value::Value;
use crate::types::*;
use bincode2;
use nanoid::nanoid;
use easy_error::{Error, bail};

impl Value {
    pub fn to_binary(&self) -> Result<Vec<u8>, Error> {
        if self.dt == JSON {
            let str_json = match self.conv(STRING) {
                Ok(str_json) => str_json,
                Err(err) => {
                    bail!("to_binary() returns: {}", err);
                }
            };
            let str_data = match str_json.cast_string() {
                Ok(str_data) => str_data,
                Err(err) => {
                    bail!("to_binary() returns: {}", err);
                }
            };
            let wrapped_json = Value::json_wrap(str_data);
            match bincode2::serialize(&wrapped_json) {
                Ok(res) => return Ok(res),
                Err(err) => {
                    bail!("bincode2::serialize() returns {}", err);
                }
            }
        } else {
            match bincode2::serialize(self) {
                Ok(res) => return Ok(res),
                Err(err) => {
                    bail!("bincode2::serialize() returns {}", err);
                }
            }
        }
    }
    pub fn compile(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if self.dt != LAMBDA {
            return Err("Attempt to compile non-lambda object".into());
        }
        match self.to_binary() {
            Ok(buffer) => {
                return Ok(buffer);
            }
            Err(err) => {
                return Err(format!("Error compiling lambda object: {}", err).into());
            }
        }
    }
    pub fn from_binary(data: Vec<u8>) -> Result<Value, Error> {
        match bincode2::deserialize::<Value>(&data) {
            Ok(value) => {
                if value.dt == JSON_WRAPPED {
                    match value.cast_string() {
                        Ok(str_data) => {
                            match serde_json::from_str(&str_data) {
                                Ok(json_val) => {
                                    return Ok(Value::json(json_val));
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
                } else {
                    return Ok(value)
                }
            }
            Err(err) => {
                bail!("{}", err);
            }
        }
    }
    pub fn wrap(&self) -> Result<Value, Error> {
        match self.to_binary() {
            Ok(res) => {
                return Result::Ok(Value::make_envelope(res));
            }
            Err(err) => bail!("{}", err),
        }
    }
    pub fn unwrap(&self) -> Result<Value, Error> {
        if self.dt != ENVELOPE {
            bail!("You requested to unwrap a non-envelope object");
        }
        match &self.data {
            Val::Binary(data) => {
                match Value::from_binary(data.clone()) {
                    Ok(mut res) => {
                        res.id = nanoid!();
                        return Result::Ok(res);
                    }
                    Err(err) => bail!("{}", err),
                }
            }
            _ => bail!("Unwrappable object {}", self.id),
        }
    }
}
