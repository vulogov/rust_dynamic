#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_dynamic::types::*;

    #[test]
    fn test_map_float() {
        let mut v = Value::from(42.0).unwrap();
        v = v.map_float(f64::sin);
        assert_eq!(v.cast_float().unwrap(), f64::sin(42.0));
    }
    #[test]
    fn test_map_float_value() {
        fn comp_sin(value: Value) -> Value {
            match value.data {
                Val::F64(f_val) => {
                    return Value::from_float(f64::sin(f_val));
                }
                _ => return value,
            }
        }
        let mut v = Value::from(42.0).unwrap();
        v = v.map_value(comp_sin);
        assert_eq!(v.cast_float().unwrap(), f64::sin(42.0));
    }
}
