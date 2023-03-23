# What's new in 0.3.0

* More test cases
* Mathematical operation Add/Sub/Mul/Div for FLOAT and INTEGER types
* String + String concatenation
* String * NUMBER, NUMBER * String - String multiplication
* Value.len() for the BINARY returned the size of the binary object
* Value::binary() creates an empty binary object
* Value.wrap() wrapping current object into it's binary representation and produces new Value of BINARY type that containing binary representation of wrapped object
* Value.unwrap() for a binary object causes unwrapping enclosed binary representation of other object and returning that object. Object ID will be regenerated.
