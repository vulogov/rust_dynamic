use crate::value::Value;
use crate::types::*;

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match &self.data {
            Val::I64(i_val_self) => {
                match &other.data {
                    Val::I64(i_val_other) => {
                        i_val_self == i_val_other
                    }
                    Val::F64(f_val_other) => {
                        *i_val_self == *f_val_other as i64
                    }
                    _ => return self.id == other.id,
                }
            }
            Val::F64(f_val_self) => {
                match &other.data {
                    Val::F64(f_val_other) => {
                        f_val_self == f_val_other
                    }
                    Val::I64(i_val_other) => {
                        *f_val_self == *i_val_other as f64
                    }
                    _ => return self.id == other.id,
                }
            }
            Val::String(s_val_self) => {
                match &other.data {
                    Val::String(s_val_other) => {
                        s_val_self == s_val_other
                    }
                    _ => return self.id == other.id,
                }
            }
            _ => return self.id == other.id,
        }
    }
}

impl Eq for Value {

}
