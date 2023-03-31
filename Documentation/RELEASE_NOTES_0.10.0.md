# What's new in 0.10.0

* More test cases
* Support for INTEGER (i64) and FLOAT (f64) Complex Number data type
* Value::from() support Complex<i64> and Complex<f64>
* Value::from_complex_int() and Value::from_complex_float(O) creating complex dynamic datatype from complex int and complex float directly
* Value.cast_complex_int() - casting Complex<i64>
* Value.cast_complex_float() - casting Complex<f64>
* == operator for CINTEGER and CFLOAT
* Add/Sub/Mul/Div for CINTEGER and CFLOAT
* >=/<=/</> operations for CINTEGER and CFLOAT
* New data-type METRICS representing a Vec of samples of TIMESTAMP->F64.
* Value.push() support for METRICS
* Iterator supported for METRICS
* Value.len() supported for METRICS
* Value.cast_metrics() - casting Vec<Metric>
* Value.export_float() - supports METRICS
