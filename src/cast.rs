use std::collections::hash_map::HashMap;
use num::complex::Complex;
use crate::value::Value;
use crate::metric::Metric;
use crate::error::BundError;
use crate::types::*;

impl Value {
    pub fn cast_float(&self) -> Result<f64, Box<dyn std::error::Error>> {
        match self.data {
            Val::F64(f_val) => {
                return Result::Ok(f_val);
            }
            _ => return Err(format!("This Dynamic type is not float: {}", self.dt).into()),
        }
    }
    pub fn cast_int(&self) -> Result<i64, Box<dyn std::error::Error>> {
        match self.data {
            Val::I64(i_val) => {
                return Result::Ok(i_val);
            }
            _ => return Err("This Dynamic type is not integer".into()),
        }
    }
    pub fn cast_bool(&self) -> Result<bool, Box<dyn std::error::Error>> {
        match self.data {
            Val::Bool(b_val) => {
                return Result::Ok(b_val);
            }
            _ => return Err("This Dynamic type is not bool".into()),
        }
    }
    pub fn cast_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        match &self.data {
            Val::String(s_val) => {
                return Result::Ok(s_val.to_string());
            }
            _ => return Err("This Dynamic type is not string".into()),
        }
    }
    pub fn cast_bin(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        match &self.data {
            Val::Binary(b_val) => {
                return Result::Ok(b_val.to_vec());
            }
            _ => return Err("This Dynamic type is not binary".into()),
        }
    }
    pub fn cast_error(&self) -> Result<BundError, Box<dyn std::error::Error>> {
        match &self.data {
            Val::Error(e_val) => {
                return Result::Ok(e_val.clone());
            }
            _ => return Err("This Dynamic type is not error".into()),
        }
    }
    pub fn cast_list(&self) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        if self.dt != LIST && self.dt != PAIR {
            return Err(format!("This is not a LIST/PAIR value but {}", &self.dt).into());
        }
        match &self.data {
            Val::List(l_val) => {
                return Result::Ok(l_val.clone());
            }
            _ => return Err("This Dynamic type is not list".into()),
        }
    }
    pub fn cast_lambda(&self) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        if self.dt != LAMBDA {
            return Err(format!("This is not a LAMBDA value but {}", &self.dt).into());
        }
        match &self.data {
            Val::Lambda(l_val) => {
                return Result::Ok(l_val.clone());
            }
            _ => return Err("This Dynamic type is not lambda".into()),
        }
    }
    pub fn cast_pair(&self) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        if self.dt != PAIR {
            return Err(format!("This is not a PAIR value but {}", &self.dt).into());
        }
        match &self.data {
            Val::List(l_val) => {
                return Result::Ok(l_val.clone());
            }
            _ => return Err("This Dynamic type is not pair".into()),
        }
    }
    pub fn cast_pair_x(&self) -> Result<Value, Box<dyn std::error::Error>> {
        if self.dt != PAIR {
            return Err(format!("This is not a PAIR value but {}", &self.dt).into());
        }
        match &self.data {
            Val::List(l_val) => {
                return Result::Ok(l_val[0].clone());
            }
            _ => return Err("This Dynamic type is not pair".into()),
        }
    }
    pub fn cast_pair_y(&self) -> Result<Value, Box<dyn std::error::Error>> {
        if self.dt != PAIR {
            return Err(format!("This is not a PAIR value but {}", &self.dt).into());
        }
        match &self.data {
            Val::List(l_val) => {
                return Result::Ok(l_val[1].clone());
            }
            _ => return Err("This Dynamic type is not pair".into()),
        }
    }
    pub fn cast_result(&self) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        if self.dt != RESULT {
            return Err(format!("This is not a LIST value but {}", &self.dt).into());
        }
        match &self.data {
            Val::List(l_val) => {
                return Result::Ok(l_val.clone());
            }
            _ => return Err("This Dynamic type is not result".into()),
        }
    }
    pub fn cast_dict(&self) -> Result<HashMap<String,Value>, Box<dyn std::error::Error>> {
        match &self.data {
            Val::Map(m_val) => {
                return Result::Ok(m_val.clone());
            }
            _ => return Err("This Dynamic type is not dict".into()),
        }
    }
    pub fn cast_timestamp(&self) -> Result<u128, Box<dyn std::error::Error>> {
        if self.dt != TIME {
            return Err("This Dynamic type is not TIME".into());
        }
        match &self.data {
            Val::Time(t_val) => {
                return Result::Ok(t_val.clone());
            }
            _ => return Err("This Dynamic type is not TIME".into()),
        }
    }
    pub fn cast_complex_int(&self) -> Result<Complex<i64>, Box<dyn std::error::Error>> {
        if self.dt != CINTEGER {
            return Err(format!("This Dynamic type is not CINTEGER: {}", &self.dt).into());
        }
        match &self.data {
            Val::List(l_val) => {
                let res = Complex::new(l_val[0].cast_int().unwrap(), l_val[1].cast_int().unwrap());
                return Result::Ok(res);
            }
            _ => return Err("This Dynamic type is not CINTEGER".into()),
        }
    }
    pub fn cast_complex_float(&self) -> Result<Complex<f64>, Box<dyn std::error::Error>> {
        if self.dt != CFLOAT {
            return Err(format!("This Dynamic type is not CFLOAT: {}", &self.dt).into());
        }
        match &self.data {
            Val::List(l_val) => {
                let res = Complex::new(l_val[0].cast_float().unwrap(), l_val[1].cast_float().unwrap());
                return Result::Ok(res);
            }
            _ => return Err("This Dynamic type is not CFLOAT".into()),
        }
    }
    pub fn cast_metrics(&self) -> Result<Vec<Metric>, Box<dyn std::error::Error>> {
        if self.dt != METRICS {
            return Err("This Dynamic type is not METRICS".into());
        }
        match &self.data {
            Val::Metrics(m_val) => {
                return Result::Ok(m_val.clone());
            }
            _ => return Err("This Dynamic type is not METRICS".into()),
        }
    }
    pub fn cast_json(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        if self.dt != JSON {
            return Err("This Dynamic type is not JSON".into());
        }
        match &self.data {
            Val::Json(j_val) => {
                return Result::Ok(j_val.clone());
            }
            _ => return Err("This Dynamic type is not JSON".into()),
        }
    }
    pub fn cast_queue(&self) -> Result<Value, Box<dyn std::error::Error>> {
        if self.dt != QUEUE {
            return Err(format!("This is not a FIFO value but {}", &self.dt).into());
        }
        if self.len() == 0 {
            return Err("Queue is empty".into());
        }
        match &self.data {
             Val::Queue(q_val) => {
                 return Result::Ok(q_val[0].clone());
             }
             _ => return Err("This Dynamic type is not queue".into()),
        }
    }
    pub fn cast_fifo(&self) -> Result<Value, Box<dyn std::error::Error>> {
        if self.dt != FIFO {
            return Err(format!("This is not a FIFO value but {}", &self.dt).into());
        }
        if self.len() == 0 {
            return Err("Fifo is empty".into());
        }
        match &self.data {
             Val::Queue(q_val) => {
                 return Result::Ok(q_val[self.len()-1].clone());
             }
             _ => return Err("This Dynamic type is not fifo".into()),
        }
    }
    pub fn cast_operator_opcode(&self) -> Result<i32, Box<dyn std::error::Error>> {
        if self.dt != OPERATOR {
            return Err(format!("This is not a OPERATOR value but {}", &self.dt).into());
        }
        match &self.data {
             Val::Operator(val) => {
                 return Result::Ok(val.opcode as i32);
             }
             _ => return Err("This Dynamic type is not operator".into()),
        }
    }
    pub fn cast_operator_value(&self) -> Result<Value, Box<dyn std::error::Error>> {
        if self.dt != OPERATOR {
            return Err(format!("This is not a OPERATOR value but {}", &self.dt).into());
        }
        match &self.data {
             Val::Operator(val) => {
                 match Value::from_binary(val.opvalue1.clone()) {
                     Ok(opval) => {
                         return Result::Ok(opval);
                     }
                     Err(err) => {
                         return Err(format!("Error unpacking operator value: {}", err).into());
                     }
                 }
             }
             _ => return Err("This Dynamic type is not operator".into()),
        }
    }
    pub fn cast_operator_value2(&self) -> Result<Value, Box<dyn std::error::Error>> {
        if self.dt != OPERATOR {
            return Err(format!("This is not a OPERATOR value but {}", &self.dt).into());
        }
        match &self.data {
             Val::Operator(val) => {
                 match Value::from_binary(val.opvalue2.clone()) {
                     Ok(opval) => {
                         return Result::Ok(opval);
                     }
                     Err(err) => {
                         return Err(format!("Error unpacking operator value: {}", err).into());
                     }
                 }
             }
             _ => return Err("This Dynamic type is not operator".into()),
        }
    }
}
