# What's new in 0.7.0

* More test cases
* Correct comparing for the i64 wrapped values
* Correct comparing for the f64 wrapped values
* Value stores now millisecond-grade timestamp of the value creation
* New datatype TIME that internally stores time as number of nanoseconds since UNIX_EPOCH
* Value.elapsed() - return number of nanoseconds between one stored in object TIME and now
* Value.elapsed_value() - same as Value.elapsed() but returns Value
* New datatype - EXIT
* Value::exit() - create an instance of datatype EXIT
* Value.as_any() - converts Value to &dyn Any
* Value.export_float() - export single or list of values as Vec<f64>
