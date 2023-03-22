use crate::value::Value;
use crate::types::*;

impl Value {
    pub fn type_of(&self) -> u16 {
        self.dt
    }
    pub fn type_name(&self) -> &'static str {
        match self.dt {
            NONE => "None",
            BOOL => "Bool",
            INTEGER => "Integer",
            FLOAT => "Float",
            STRING => "String",
            LITERAL => "Literal",
            CALL => "Call",
            PTR => "Ptr",
            BIN => "Bin",
            ERROR => "Error",
            TOKEN => "Token",
            _ => "Unknown",
        }
    }
}
