#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_dynamic::types::*;

    #[test]
    fn test_conv_float_float() {
        let val = Value::from(42.0 as f64).unwrap().conv(FLOAT).unwrap();
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }
    #[test]
    fn test_conv_float_int() {
        let val = Value::from(42.0 as f64).unwrap().conv(INTEGER).unwrap();
        assert_eq!(val.cast_int().unwrap(), 42 as i64);
    }
    #[test]
    fn test_conv_float_bool() {
        let val = Value::from(42.0 as f64).unwrap().conv(BOOL).unwrap();
        assert_eq!(val.cast_bool().unwrap(), true);
    }
    #[test]
    fn test_conv_float_string() {
        let val = Value::from(42.0 as f64).unwrap().conv(STRING).unwrap();
        assert_eq!(val.cast_string().unwrap(), "42.0");
    }
    #[test]
    fn test_conv_float_list_check_len() {
        let val = Value::from(42.0 as f64).unwrap().conv(LIST).unwrap();
        assert_eq!(val.len(), 1);
    }
    #[test]
    fn test_conv_float_list() {
        let val = Value::from(42.0 as f64).unwrap().conv(LIST).unwrap();
        assert_eq!(val.cast_list().unwrap()[0].cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_conv_int_float() {
        let val = Value::from(42 as i64).unwrap().conv(FLOAT).unwrap();
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }
    #[test]
    fn test_conv_int_int() {
        let val = Value::from(42 as i64).unwrap().conv(INTEGER).unwrap();
        assert_eq!(val.cast_int().unwrap(), 42 as i64);
    }
    #[test]
    fn test_conv_int_bool() {
        let val = Value::from(42 as i64).unwrap().conv(BOOL).unwrap();
        assert_eq!(val.cast_bool().unwrap(), true);
    }
    #[test]
    fn test_conv_int_string() {
        let val = Value::from(42 as i64).unwrap().conv(STRING).unwrap();
        assert_eq!(val.cast_string().unwrap(), "42");
    }
    #[test]
    fn test_conv_int_list_check_len() {
        let val = Value::from(42 as i64).unwrap().conv(LIST).unwrap();
        assert_eq!(val.len(), 1);
    }
    #[test]
    fn test_conv_int_list() {
        let val = Value::from(42 as i64).unwrap().conv(LIST).unwrap();
        assert_eq!(val.cast_list().unwrap()[0].cast_int().unwrap(), 42 as i64);
    }
    #[test]
    fn test_conv_string_bool() {
        let val = Value::from("true").unwrap().conv(BOOL).unwrap();
        assert_eq!(val.cast_bool().unwrap(), true);
    }
    #[test]
    fn test_conv_string_float() {
        let val = Value::from("42.0").unwrap().conv(FLOAT).unwrap();
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }
    #[test]
    fn test_conv_string_int() {
        let val = Value::from("42").unwrap().conv(INTEGER).unwrap();
        assert_eq!(val.cast_int().unwrap(), 42 as i64);
    }
}
