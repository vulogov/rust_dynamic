# What's new in 0.19.0

* More test cases
* Value::operator() - create a single operator. Operator is a value suitable to store an opcode/opvalue for an execution in virtual machine
* Value::cast_operator_opcode() - return OPERATOR opcode
* Value::cast_operator_value() - returns the value of an OPERATOR
* Value::pop() - returns last value pushed to the (LIST | RESULT | FIFO) or None
* Value::compile() - attempts to bin-encode lambda object and return Vec<u8> or Error
