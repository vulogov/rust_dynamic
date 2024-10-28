use crate::value::Value;
use crate::types::*;

impl Value {
    pub fn cast_value_to_json(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        if self.dt == JSON {
            return Err(format!("This Dynamic type is already json: {}", self.dt).into());
        }
        let ret = match self.dt {
            INTEGER => {
                match self.cast_int() {
                    Ok(int_value) => serde_json::json!(int_value),
                    Err(err) => {
                        return Err(format!("Error casting INTEGER: {}", err).into());
                    }
                }
            }
            FLOAT => {
                match self.cast_float() {
                    Ok(float_value) => serde_json::json!(float_value),
                    Err(err) => {
                        return Err(format!("Error casting FLOAT: {}", err).into());
                    }
                }
            }
            BOOL => {
                match self.cast_bool() {
                    Ok(bool_value) => serde_json::json!(bool_value),
                    Err(err) => {
                        return Err(format!("Error casting BOOL: {}", err).into());
                    }
                }
            }
            NONE => {
                serde_json::json!(null)
            }
            LIST => {
                let mut res = serde_json::json!([]);
                match self.cast_list() {
                    Ok(l_value) => {
                        for n in l_value {
                            match n.cast_value_to_json() {
                                Ok(j_value) => {
                                    let Some(jres) = res.as_array_mut() else { return Err(format!("Error appending LIST").into()); };
                                    jres.push(j_value);
                                }
                                Err(err) => {
                                    return Err(format!("Error casting LIST: {}", err).into());
                                }
                            }
                        }
                    }
                    Err(err) => {
                        return Err(format!("Error casting LIST: {}", err).into());
                    }
                }
                res
            }
            MAP => {
                let mut res = serde_json::json!({});
                match self.cast_dict() {
                    Ok(m_value) => {
                        for k in m_value.keys() {
                            let v = match m_value.get(k) {
                                Some(v) => v,
                                None => return Err(format!("Error casting DICT").into()),
                            };
                            match v.cast_value_to_json() {
                                Ok(val) => {
                                    res.as_object_mut().unwrap().insert(k.to_string(), val);
                                }
                                Err(err) => {
                                    return Err(format!("Error casting DICT: {}", err).into());
                                }
                            }
                        }
                    }
                    Err(err) => {
                        return Err(format!("Error casting DICT: {}", err).into());
                    }
                }
                res
            }
            _ => return Err(format!("This Dynamic type is not supported for JSON: {}", self.dt).into()),
        };
        return Ok(ret);
    }
}
