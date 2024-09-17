use crate::value::Value;
use crate::types::*;

impl Value {
    pub fn type_of(&self) -> u16 {
        self.dt
    }
    pub fn type_name(&self) -> &'static str {
        match self.dt {
            NONE        => "None",
            NODATA      => "NODATA",
            BOOL        => "Bool",
            INTEGER     => "Integer",
            FLOAT       => "Float",
            STRING      => "String",
            LITERAL     => "Literal",
            CALL        => "Call",
            PTR         => "Ptr",
            BIN         => "Binary",
            LIST        => "List",
            PAIR        => "Pair",
            MAP         => "Map",
            CONFIG      => "Config",
            INFO        => "Info",
            ENVELOPE    => "Envelope",
            TIME        => "Time",
            CONTEXT     => "Context",
            ERROR       => "Error",
            TOKEN       => "Token",
            EXIT        => "Exit",
            RESULT      => "Result",
            ASSOCIATION => "Association",
            CINTEGER    => "ComplexInteger",
            CFLOAT      => "ComplexFloat",
            METRICS     => "Metrics",
            LAMBDA      => "Lambda",
            QUEUE       => "Queue",
            FIFO        => "Fifo",
            OPERATOR    => "Operator",

            _ => "Unknown",
        }
    }
}
