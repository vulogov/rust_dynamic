use crate::value::Value;
use crate::types::*;

impl Value {
    pub fn export_float(&self) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
        match &self.data {
            Val::F64(f_val) => {
                return Result::Ok(vec![*f_val]);
            }
            Val::I64(i_val) => {
                return Result::Ok(vec![*i_val as f64]);
            }
            Val::List(l_val) => {
                let mut res: Vec<f64> = Vec::new();
                for n in l_val {
                    match n.dt {
                        FLOAT => {
                            match n.cast_float() {
                                Ok(f_val) => res.push(f_val),
                                _ => {},
                            }
                        }
                        INTEGER => {
                            match n.cast_int() {
                                Ok(i_val) => res.push(i_val as f64),
                                _ => {},
                            }
                        }
                        _ => {},
                    }

                }
                return Result::Ok(res);
            }
            _ => return Err("This Dynamic type is not float".into()),
        }
    }
}
