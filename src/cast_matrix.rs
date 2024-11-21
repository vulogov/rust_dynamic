use crate::value::Value;
use crate::types::*;

impl Value {
    pub fn cast_matrix(&self) -> Result<Vec<Vec<Value>>, Box<dyn std::error::Error>> {
        if self.dt != MATRIX {
            return Err(format!("This is not a MATRIX value but {}", &self.dt).into());
        }
        match &self.data {
            Val::Matrix(m_val) => {
                return Result::Ok(m_val.clone());
            }
            _ => return Err("This Dynamic type is not matrix".into()),
        }
    }
}
