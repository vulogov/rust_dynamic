use neurons::tensor::Tensor;
use crate::value::{Value};
use crate::types::*;
use easy_error::{Error, bail};


impl Value {
    pub fn tensor(&mut self) -> Result<Tensor, Error> {
        match self.dt {
            INTEGER => {
                let int_value = match self.cast_int() {
                    Ok(int_value) => int_value,
                    Err(err) => bail!("Error casting value for INT tensor: {}", err),
                };
                return Ok(Tensor::single(vec![int_value as f32]));
            }
            FLOAT => {
                let f_value = match self.cast_float() {
                    Ok(f_value) => f_value,
                    Err(err) => bail!("Error casting value for FLOAT tensor: {}", err),
                };
                return Ok(Tensor::single(vec![f_value as f32]));
            }
            STRING => {
                let str_value = match self.cast_string() {
                    Ok(str_value) => str_value,
                    Err(err) => bail!("Error casting value for STRING tensor: {}", err),
                };
                let bytes: Vec<u8> = str_value.into_bytes();
                let mut row: Vec<f32> = Vec::new();
                for b in bytes.into_iter() {
                    row.push(b as f32);
                }
                return Ok(Tensor::double(vec![row]));
            }
            _ => {
                bail!("This datatype does not supported by TENSOR: {}", self.type_name());
            }
        }
    }
}
