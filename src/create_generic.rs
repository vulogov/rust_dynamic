use std::any::Any;
use crate::value::Value;
use crate::error::BundError;
use num::complex::Complex;

impl Value {
    pub fn from<T: Clone + 'static>(value: T) -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(f_val) = (&value as &dyn Any).downcast_ref::<f64>() {
            return Result::Ok(Value::from_float(*f_val));
        } else if let Some(f32_val) = (&value as &dyn Any).downcast_ref::<f32>() {
            return Result::Ok(Value::from_float32(*f32_val));
        } else if let Some(i_val) = (&value as &dyn Any).downcast_ref::<i64>() {
            return Result::Ok(Value::from_int(*i_val));
        } else if let Some(i32_val) = (&value as &dyn Any).downcast_ref::<i32>() {
            return Result::Ok(Value::from_int32(*i32_val));
        } else if let Some(bool_val) = (&value as &dyn Any).downcast_ref::<bool>() {
            return Result::Ok(Value::from_bool(*bool_val));
        } else if let Some(string_val) = (&value as &dyn Any).downcast_ref::<String>() {
            return Result::Ok(Value::from_string((*string_val.clone()).to_string()));
        } else if let Some(str_val) = (&value as &dyn Any).downcast_ref::<&str>() {
            return Result::Ok(Value::from_str(&*str_val));
        } else if let Some(lst_val) = (&value as &dyn Any).downcast_ref::<Vec<Value>>() {
            return Result::Ok(Value::from_list((*lst_val.clone()).to_vec()));
        } else if let Some(mat_val) = (&value as &dyn Any).downcast_ref::<Vec<Vec<Value>>>() {
            return Result::Ok(Value::from_matrix((*mat_val.clone()).to_vec()));
        } else if let Some(err_val) = (&value as &dyn Any).downcast_ref::<BundError>() {
            return Result::Ok(Value::from_error(err_val.clone()));
        } else if let Some(ci_val) = (&value as &dyn Any).downcast_ref::<Complex<i64>>() {
            return Result::Ok(Value::from_complex_int(ci_val.clone()));
        } else if let Some(cf_val) = (&value as &dyn Any).downcast_ref::<Complex<f64>>() {
            return Result::Ok(Value::from_complex_float(cf_val.clone()));
        }
        else {
            return Err("Unknown dynamic type".into());
        }
    }
}
