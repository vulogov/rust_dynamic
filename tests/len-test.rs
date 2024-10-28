#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_len_list() {
        let v = Value::from(vec![Value::new(), Value::new()]).unwrap();
        assert_eq!(v.len(), 2);
    }
    #[test]
    fn test_len_nodata() {
        let v = Value::nodata();
        assert_eq!(v.len(), 0);
    }
    #[test]
    fn test_len_regular() {
        let v = Value::from(42 as i64).unwrap();
        assert_eq!(v.len(), 2);
    }
    #[test]
    fn test_len_lambda() {
        let val = Value::to_lambda(vec![Value::from_int(42)]);
        assert_eq!(val.len(), 1);
    }
    #[test]
    fn test_len_textbuffer() {
        let v = Value::text_buffer("".to_string());
        assert_eq!(v.len(), 0);
    }
    #[test]
    fn test_len_string() {
        let v = Value::from_string("Hello");
        assert_eq!(v.len(), 5);
    }
    #[test]
    fn test_len_float() {
        let v = Value::from_float(3.14);
        assert_eq!(v.len(), 4);
    }
    #[test]
    fn test_len_json_array() {
        let v = Value::json(serde_json::json!([3.14, 42]));
        assert_eq!(v.len(), 2);
    }
}
