use crate::value::Value;
use crate::types::*;

impl Value {
    pub fn wrap_json(&self) -> Result<Value, Box<dyn std::error::Error>> {
        if ! self.dt == JSON {
            return Err(format!("This Dynamic type is not json: {}", self.dt).into());
        }
        match &self.data {
            Val::Json(j_val) => {
                if j_val.is_string() {
                    match j_val.as_str() {
                        Some(j_string) => {
                            return Ok(Value::from_string(j_string));
                        }
                        None => {
                            return Err(format!("This JSON is having a data that is not STRING").into());
                        }
                    }
                } else if j_val.is_i64() {
                    match j_val.as_i64() {
                        Some(j_int) => {
                            return Ok(Value::from_int(j_int));
                        }
                        None => {
                            return Err(format!("This JSON is having a data that is not INT").into());
                        }
                    }
                } else if j_val.is_f64() {
                    match j_val.as_f64() {
                        Some(j_float) => {
                            return Ok(Value::from_float(j_float));
                        }
                        None => {
                            return Err(format!("This JSON is having a data that is not FLOAT").into());
                        }
                    }
                } else if j_val.is_null() {
                    return Ok(Value::none());
                } else if j_val.is_boolean() {
                    match j_val.as_bool() {
                        Some(j_bool) => {
                            return Ok(Value::from_bool(j_bool));
                        }
                        None => {
                            return Err(format!("This JSON is having a data that is not BOOL").into());
                        }
                    }
                } else if j_val.is_array() {
                    let mut res = Value::list();
                    match j_val.as_array() {
                        Some(j_arr) => {
                            for v in j_arr {
                                let jv = Value::json(v.clone());
                                match jv.cast_json_to_value() {
                                    Ok(ja_val) => {
                                        res.push(ja_val);
                                    }
                                    Err(err) => {
                                        return Err(format!("JSON returns error during casting: {}", err).into());
                                    }
                                }
                            }
                        }
                        None => {
                            return Err(format!("This JSON is having a data that is not ARRAY").into());
                        }
                    }
                    return Ok(res);
                } else if j_val.is_object() {
                    let mut res = Value::dict();
                    match j_val.as_object() {
                        Some(j_val_obj) => {
                            for n in j_val_obj.keys() {
                                let j_val_value = match j_val_obj.get(n) {
                                    Some(j_val_value) => j_val_value.clone(),
                                    None => serde_json::json!(null),
                                };
                                let jv = Value::json(j_val_value);
                                match jv.cast_json_to_value() {
                                    Ok(ja_val) => {
                                        res.set(n, ja_val);
                                    }
                                    Err(err) => {
                                        return Err(format!("JSON returns error during casting: {}", err).into());
                                    }
                                }
                            }
                        }
                        None => {
                            return Err(format!("This JSON is having a data that is not OBJECT").into());
                        }
                    }
                    return Ok(res);
                } else {
                    return Err(format!("This JSON is having a data that is not supportable").into());
                }
            }
            _ => return Err(format!("This Dynamic type is not JSON: {}", self.dt).into()),
        }
    }
}
