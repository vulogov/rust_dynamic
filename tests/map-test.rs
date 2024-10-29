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
        v = v.fmap(comp_sin);
        assert_eq!(v.cast_float().unwrap(), f64::sin(42.0));
    }

    #[test]
    fn test_map_json_array_value() {
        fn comp_sin(value: Value) -> Value {
            match value.data {
                Val::F64(f_val) => {
                    return Value::from_float(f64::sin(f_val));
                }
                _ => return value,
            }
        }
        let mut v = Value::json(serde_json::json!([42.0]));
        let v_json_array = v.fmap(comp_sin);
        let v_list = v_json_array.cast_list().unwrap();
        assert_eq!(v_list[0].cast_float().unwrap(), f64::sin(42.0));
    }

    #[test]
    fn test_map_float_map() {
        fn comp_sin(value: Value) -> Value {
            match value.data {
                Val::F64(f_val) => {
                    return Value::from_float(f64::sin(f_val));
                }
                _ => return value,
            }
        }
        let mut v = Value::association("answer", Value::from(42.0).unwrap());
        v = v.fmap(comp_sin);
        let val = v.get("answer".to_string()).unwrap();
        assert_eq!(val.cast_float().unwrap(), f64::sin(42.0));
    }
    #[test]
    fn test_map_float_map_but_param_string() {
        fn comp_sin(value: Value) -> Value {
            match value.data {
                Val::F64(f_val) => {
                    return Value::from_float(f64::sin(f_val));
                }
                _ => return value,
            }
        }
        let mut v = Value::association("answer".to_string(), Value::from(42.0).unwrap());
        v = v.fmap(comp_sin);
        let val = v.get("answer").unwrap();
        assert_eq!(val.cast_float().unwrap(), f64::sin(42.0));
    }
}
