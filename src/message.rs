use crate::value::{Value};
use crate::types::*;

impl Value {
    pub fn message_add_to<N: AsRef<str>>(&mut self, to: N) -> Self {
        match self.dt {
            MESSAGE => {
                let mut to_vec = match self.get("to") {
                    Ok(to_vec) => to_vec,
                    Err(_) => Value::list(),
                };
                to_vec.push(Value::from_string(to));
                self.set("to", to_vec);
                return self.clone();
            }
            _ => return self.clone(),
        }
    }

    pub fn message_add_payload(&mut self, data: Value) -> Self {
        match self.dt {
            MESSAGE => {
                let mut p_vec = match self.get("payload") {
                    Ok(to_vec) => to_vec,
                    Err(_) => Value::list(),
                };
                p_vec.push(data);
                self.set("payload", p_vec);
                return self.clone();
            }
            _ => return self.clone(),
        }
    }

}
