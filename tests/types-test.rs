#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_dynamic::error::BundError;

    #[test]
    fn test_none() {
        assert_eq!(rust_dynamic::types::NONE, 0);
    }
    #[test]
    fn test_create_none() {
        let v = Value::new();
        assert_eq!(v.type_of(), rust_dynamic::types::NONE);
    }
    #[test]
    fn test_create_none_type_name() {
        let v = Value::new();
        assert_eq!(v.type_name(), "None");
    }
    #[test]
    fn test_create_float_type_name() {
        let v = Value::from(42.0).unwrap();
        assert_eq!(v.type_name(), "Float");
    }
    #[test]
    fn test_create_int_type_name() {
        let v = Value::from(42 as i64).unwrap();
        assert_eq!(v.type_name(), "Integer");
    }
    #[test]
    fn test_create_float32_type_name() {
        let v = Value::from(42.0 as f32).unwrap();
        assert_eq!(v.type_name(), "Float");
    }
    #[test]
    fn test_create_int32_type_name() {
        let v = Value::from(42 as i32).unwrap();
        assert_eq!(v.type_name(), "Integer");
    }
    #[test]
    fn test_create_bool_type_name() {
        let v = Value::from(true).unwrap();
        assert_eq!(v.type_name(), "Bool");
    }
    #[test]
    fn test_create_string_type_name() {
        let v = Value::from("Hello".to_string()).unwrap();
        assert_eq!(v.type_name(), "String");
    }
    #[test]
    fn test_create_str_type_name() {
        let v = Value::from("Hello").unwrap();
        assert_eq!(v.type_name(), "String");
    }
    #[test]
    fn test_create_nodata_type_name() {
        let v = Value::nodata();
        assert_eq!(v.type_name(), "NODATA");
    }
    #[test]
    fn test_create_list_type_name() {
        let v = Value::from(vec![Value::new(), Value::new()]).unwrap();
        assert_eq!(v.type_name(), "List");
    }
    #[test]
    fn test_create_error_type_name() {
        let v = Value::from(BundError::new("Hello".to_string(), "World".to_string())).unwrap();
        assert_eq!(v.type_name(), "Error");
    }
    #[test]
    fn test_create_pair_type_name() {
        let v = Value::from_pair((Value::new(), Value::new()));
        assert_eq!(v.type_name(), "Pair");
    }
}
