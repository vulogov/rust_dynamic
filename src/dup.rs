use crate::value::Value;
use nanoid::nanoid;

impl Value {
    pub fn dup(&self) -> Result<Value, bincode2::Error> {
        match self.to_binary() {
            Ok(bin_rep) => {
                match Value::from_binary(bin_rep) {
                    Ok(mut res) => {
                        res.id = nanoid!();
                        return Result::Ok(res);
                    }
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(err),
        }
    }
}
