use dtoa;
use itoa;
use rustils;
use crate::value::Value;
use crate::types::*;

fn value_float_conversion(t: u16, ot: u16, val: f64) -> Result<Value, Box<dyn std::error::Error>> {
    if ot != FLOAT {
        return Err(format!("Source value is not FLOAT but {:?} and not suitable for conversion", &ot).into());
    }
    match t {
        FLOAT => {
            return Result::Ok(Value::from(val as f64).unwrap());
        }
        INTEGER => {
            return Result::Ok(Value::from(val as i64).unwrap());
        }
        BOOL => {
            if val == 0.0 as f64 {
                return Result::Ok(Value::from(false).unwrap());
            }
            return Result::Ok(Value::from(true).unwrap());
        }
        STRING => {
            let mut buffer = dtoa::Buffer::new();
            return Result::Ok(Value::from_string(buffer.format(val).to_string()));
        }
        LIST => {
            return Result::Ok(Value::from(vec![Value::from(val as f64).unwrap()]).unwrap());
        }
        _ => Err(format!("Can not convert float to {:?}", &t).into()),
    }
}

fn value_integer_conversion(t: u16, ot: u16, val: i64) -> Result<Value, Box<dyn std::error::Error>> {
    if ot != INTEGER {
        return Err(format!("Source value is not INTEGER but {:?} and not suitable for conversion", &ot).into());
    }
    match t {
        FLOAT => {
            return Result::Ok(Value::from(val as f64).unwrap());
        }
        INTEGER => {
            return Result::Ok(Value::from(val as i64).unwrap());
        }
        BOOL => {
            if val == 0 as i64 {
                return Result::Ok(Value::from(false).unwrap());
            }
            return Result::Ok(Value::from(true).unwrap());
        }
        STRING => {
            let mut buffer = itoa::Buffer::new();
            return Result::Ok(Value::from_string(buffer.format(val).to_string()));
        }
        LIST => {
            return Result::Ok(Value::from(vec![Value::from(val as i64).unwrap()]).unwrap());
        }
        _ => Err(format!("Can not convert integer to {:?}", &t).into()),
    }
}

fn value_string_conversion(t: u16, ot: u16, val: &String) -> Result<Value, Box<dyn std::error::Error>> {
    if ot != STRING {
        return Err(format!("Source value is not STRING but {:?} and not suitable for conversion", &ot).into());
    }
    match t {
        FLOAT => {
            match rustils::parse::double::string_to_f64_res(val.to_string()) {
                Ok(f_res) => {
                    return Result::Ok(Value::from(f_res as f64).unwrap());
                }
                Err(err) => Err(format!("Can not convert string to float {:?}", err).into()),
            }
        }
        INTEGER => {
            match rustils::parse::long::string_to_i64_res(val.to_string()) {
                Ok(i_res) => {
                    return Result::Ok(Value::from(i_res as i64).unwrap());
                }
                Err(err) => Err(format!("Can not convert string to integer {:?}", err).into()),
            }
        }
        BOOL => {
            return Result::Ok(Value::from_bool(rustils::parse::boolean::string_to_bool(val.to_string())));
        }
        STRING => {
            return Result::Ok(Value::from_string(val.to_string()));
        }
        LIST => {
            return Result::Ok(Value::from(vec![Value::from(val.to_string()).unwrap()]).unwrap());
        }
        _ => Err(format!("Can not convert string to {:?}", &t).into()),
    }
}

fn value_bool_conversion(t: u16, ot: u16, val: bool) -> Result<Value, Box<dyn std::error::Error>> {
    if ot != BOOL {
        return Err(format!("Source value is not BOOL but {:?} and not suitable for conversion", &ot).into());
    }
    match t {
        FLOAT => {
            if val {
                return Result::Ok(Value::from_float(1.0));
            } else {
                return Result::Ok(Value::from_float(0.0));
            }
        }
        INTEGER => {
            if val {
                return Result::Ok(Value::from_int(1));
            } else {
                return Result::Ok(Value::from_int(0));
            }
        }
        BOOL => {
            return Result::Ok(Value::from_bool(val));
        }
        STRING => {
            return Result::Ok(Value::from_string(format!("{}",val)));
        }
        LIST => {
            return Result::Ok(Value::from(vec![Value::from_bool(val)]).unwrap());
        }
        _ => Err(format!("Can not convert string to {:?}", &t).into()),
    }
}

impl Value {
    pub fn conv(&self, t: u16) -> Result<Self, Box<dyn std::error::Error>> {
        match &self.data {
            Val::F64(f_val) => value_float_conversion(t, self.dt, *f_val),
            Val::I64(i_val) => value_integer_conversion(t, self.dt, *i_val),
            Val::String(s_val) => value_string_conversion(t, self.dt, &s_val),
            Val::Bool(b_val) => value_bool_conversion(t, self.dt, *b_val),
            _ => Err(format!("Can not convert float from {:?}", &self.dt).into()),
        }
    }
}
