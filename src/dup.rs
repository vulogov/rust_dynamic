use crate::value::Value;
use nanoid::nanoid;
use easy_error::{Error, bail};

impl Value {
    pub fn dup(&self) -> Result<Value, Error> {
        match self.to_binary() {
            Ok(bin_rep) => {
                match Value::from_binary(bin_rep) {
                    Ok(mut res) => {
                        res.id = nanoid!();
                        return Result::Ok(res);
                    }
                    Err(err) => bail!("dup() returns: {}", err),
                }
            }
            Err(err) => bail!("dup() returns: {}", err),
        }
    }
}
