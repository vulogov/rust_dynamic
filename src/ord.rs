use crate::value::Value;
use crate::types::*;
use std::cmp::Ordering;

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
    fn lt(&self, other: &Self) -> bool {
        match self.data {
            Val::I64(i_val_self) => {
                match other.data {
                    Val::I64(i_val_other) => {
                        i_val_self < i_val_other
                    }
                    _ => return true,
                }
            }
            Val::F64(f_val_self) => {
                match other.data {
                    Val::F64(f_val_other) => {
                        f_val_self < f_val_other
                    }
                    _ => return true,
                }
            }
            _ => return true,
        }
    }
    fn le(&self, other: &Self) -> bool {
        match self.data {
            Val::I64(i_val_self) => {
                match other.data {
                    Val::I64(i_val_other) => {
                        i_val_self <= i_val_other
                    }
                    _ => return true,
                }
            }
            Val::F64(f_val_self) => {
                match other.data {
                    Val::F64(f_val_other) => {
                        f_val_self <= f_val_other
                    }
                    _ => return true,
                }
            }
            _ => return true,
        }
    }
    fn gt(&self, other: &Self) -> bool {
        match self.data {
            Val::I64(i_val_self) => {
                match other.data {
                    Val::I64(i_val_other) => {
                        i_val_self > i_val_other
                    }
                    _ => return true,
                }
            }
            Val::F64(f_val_self) => {
                match other.data {
                    Val::F64(f_val_other) => {
                        f_val_self > f_val_other
                    }
                    _ => return true,
                }
            }
            _ => return true,
        }
    }
    fn ge(&self, other: &Self) -> bool {
        match self.data {
            Val::I64(i_val_self) => {
                match other.data {
                    Val::I64(i_val_other) => {
                        i_val_self >= i_val_other
                    }
                    _ => return true,
                }
            }
            Val::F64(f_val_self) => {
                match other.data {
                    Val::F64(f_val_other) => {
                        f_val_self >= f_val_other
                    }
                    _ => return true,
                }
            }
            _ => return true,
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match &self.data {
            Val::I64(i_val_self) => {
                match &other.data {
                    Val::I64(i_val_other) => {
                        i_val_self.cmp(&i_val_other)
                    }
                    _ => return self.id.cmp(&other.id),
                }
            }
            Val::String(s_val_self) => {
                match &other.data {
                    Val::String(s_val_other) => {
                        s_val_self.cmp(&s_val_other)
                    }
                    _ => return self.id.cmp(&other.id),
                }
            }
            _ => return self.id.cmp(&other.id),
        }
    }
}
