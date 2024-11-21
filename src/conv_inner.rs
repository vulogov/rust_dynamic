use crate::types::*;
use crate::value::Value;


impl Value {
    pub fn conv_inner(&self, t: u16) -> Result<Self, Box<dyn std::error::Error>> {
        match &self.data {
            Val::Matrix(m_val) => {
                let mut res: Vec<Vec<Value>> = Vec::new();
                for r in m_val {
                    let mut row: Vec<Value> = Vec::new();
                    for v in r {
                        match v.conv(t) {
                            Ok(value) => {
                                row.push(value);
                            }
                            Err(err) => {
                                return Err(format!("Can not convert element of matrix row: {}", err).into());
                            }
                        }
                    }
                    res.push(row);
                }
                return Result::Ok(Value::from_matrix(res));
            }
            Val::List(l_val) => {
                let mut res: Vec<Value> = Vec::new();
                for v in l_val {
                    match v.conv(t) {
                        Ok(value) => {
                            res.push(value);
                        }
                        Err(err) => {
                            return Err(format!("Can not convert element of list: {}", err).into());
                        }
                    }
                }
                return Result::Ok(Value::from_list(res));
            }
            _ => {
                return self.conv(t);
            }
        }
    }
}
