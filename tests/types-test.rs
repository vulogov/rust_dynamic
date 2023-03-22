#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;

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
}
