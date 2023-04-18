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
    #[test]
    fn test_conv_bool_list_check_len() {
        let val = Value::from(true).unwrap().conv(LIST).unwrap();
        assert_eq!(val.len(), 1);
    }
    #[test]
    fn test_conv_bool_list() {
        let val = Value::from(true).unwrap().conv(LIST).unwrap();
        assert!(val.cast_list().unwrap()[0].cast_bool().unwrap());
    }
    #[test]
    fn test_conv_bool_float() {
        let val = Value::from(true).unwrap().conv(FLOAT).unwrap();
        assert_eq!(val.cast_float().unwrap(), 1.0 as f64);
    }
    #[test]
    fn test_conv_list_string() {
        let val = Value::from(vec![Value::from_int(42)]).unwrap().conv(STRING).unwrap();
        assert_eq!(val.cast_string().unwrap(), "[ 42 :: ]");
    }
    #[test]
    fn test_conv_list_bool() {
        let val = Value::from(vec![Value::from_int(42)]).unwrap().conv(BOOL).unwrap();
        assert!(val.cast_bool().unwrap());
    }
    #[test]
    fn test_conv_map_string() {
        let val = Value::dict()
            .set("answer".to_string(), Value::from(42 as i64).unwrap())
            .conv(STRING).unwrap();
        assert_eq!(val.cast_string().unwrap(), "{ answer=42 :: }");
    }
    #[test]
    fn test_conv_map_list() {
        let val = Value::dict()
            .set("answer".to_string(), Value::from(42 as i64).unwrap())
            .conv(LIST).unwrap();
        assert_eq!(val.cast_list().unwrap()[0].cast_list().unwrap()[1].cast_int().unwrap(), 42 as i64);
    }
    #[test]
    fn test_conv_list_map() {
        let val = Value::from(vec![Value::from_int(42)]).unwrap().conv(MAP).unwrap();
        let val2 = val.get("0".to_string()).unwrap();
        assert_eq!(val2.cast_int().unwrap(), 42 as i64);
    }
    #[test]
    fn test_conv_lambda_string() {
        let val = Value::to_lambda(vec![Value::from_int(42)]).conv(STRING).unwrap();
        assert_eq!(val.cast_string().unwrap(), "lambda[ 42 :: ]");
    }
}
