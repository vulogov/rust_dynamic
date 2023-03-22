use crate::value::Value;
use crate::error::BundError;
use crate::types::*;

impl Value {
    pub fn cast_float(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        match self.data {
            Val::F64(f_val) => {
                return Result::Ok(f_val);
            }
            _ => return Err("This Dynamic type is not float".into()),
        }
    }
    pub fn cast_int(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        match self.data {
            Val::I64(i_val) => {
                return Result::Ok(i_val);
            }
            _ => return Err("This Dynamic type is not integer".into()),
        }
    }
    pub fn cast_bool(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        match self.data {
            Val::Bool(b_val) => {
                return Result::Ok(b_val);
            }
            _ => return Err("This Dynamic type is not bool".into()),
        }
    }
    pub fn cast_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        match &self.data {
            Val::String(s_val) => {
                return Result::Ok(s_val.to_string());
            }
            _ => return Err("This Dynamic type is not string".into()),
        }
    }
    pub fn cast_bin(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        match &self.data {
            Val::Binary(b_val) => {
                return Result::Ok(b_val.to_vec());
            }
            _ => return Err("This Dynamic type is not binary".into()),
        }
    }
    pub fn cast_error(&mut self) -> Result<BundError, Box<dyn std::error::Error>> {
        match &self.data {
            Val::Error(e_val) => {
                return Result::Ok(e_val.clone());
            }
            _ => return Err("This Dynamic type is not error".into()),
        }
    }
    pub fn cast_list(&mut self) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        match &self.data {
            Val::List(l_val) => {
                return Result::Ok(l_val.clone());
            }
            _ => return Err("This Dynamic type is not list".into()),
        }
    }
}
