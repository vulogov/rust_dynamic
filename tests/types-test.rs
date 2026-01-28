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
    fn test_create_embedding_type_name() {
        let v = Value::embedding(vec![42.0]);
        assert_eq!(v.type_name(), "Embedding");
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
    fn test_create_matrix_type_name() {
        let v = Value::from(vec![vec![Value::new(), Value::new()]]).unwrap();
        assert_eq!(v.type_name(), "Matrix");
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
    #[test]
    fn test_create_ptr_type_name() {
        let v = Value::ptr("name".to_string(), vec![]);
        assert_eq!(v.type_name(), "Ptr");
    }
    #[test]
    fn test_create_call_type_name() {
        let v = Value::call("name".to_string(), vec![]);
        assert_eq!(v.type_name(), "Call");
    }
    #[test]
    fn test_create_float_nan() {
        let v = Value::nan();
        assert!(v.cast_float().unwrap().is_nan());
    }
    #[test]
    fn test_create_float_inf1() {
        let v = Value::inf();
        assert!(v.cast_float().unwrap().is_infinite());
    }
    #[test]
    fn test_create_float_inf2() {
        let v = Value::negative_inf();
        assert!(v.cast_float().unwrap().is_infinite());
    }
    #[test]
    fn test_create_float_pi() {
        use std::f64::consts::PI;

        let v = Value::pi();
        assert_eq!(v.cast_float().unwrap(), PI);
    }
    #[test]
    fn test_create_float_e() {
        use std::f64::consts::E;

        let v = Value::e();
        assert_eq!(v.cast_float().unwrap(), E);
    }
    #[test]
    fn test_create_binary_type_name() {
        let v = Value::binary();
        assert_eq!(v.type_name(), "Binary");
    }
    #[test]
    fn test_create_binary_type() {
        let v = Value::binary();
        assert_eq!(v.len(), 0);
    }
    #[test]
    fn test_create_dict_type_name() {
        let v = Value::dict();
        assert_eq!(v.type_name(), "Map");
    }
    #[test]
    fn test_create_dict_type() {
        let v = Value::dict();
        assert_eq!(v.len(), 0);
    }
    #[test]
    fn test_create_curry_type() {
        let v = Value::ptr_curry("Test", "TestPtr", Vec::new());
        assert_eq!(v.len(), 4);
    }
    #[test]
    fn test_create_message_type() {
        let v = Value::message(Value::from_string("ABC"), Value::from_string("CDE"), Value::from_string("GHI"));
        assert_eq!(v.type_name(), "Message");
    }
    #[test]
    fn test_create_conditional_type() {
        let v = Value::conditional();
        assert_eq!(v.type_name(), "Conditional");
    }
    #[test]
    fn test_create_exit_type() {
        let v = Value::exit();
        assert_eq!(v.type_name(), "Exit");
    }
    #[test]
    fn test_create_lambda_type() {
        let v = Value::lambda();
        assert_eq!(v.type_name(), "Lambda");
    }
    #[test]
    fn test_create_queue_type() {
        let v = Value::queue();
        assert_eq!(v.type_name(), "Queue");
    }
    #[test]
    fn test_create_fifo_type() {
        let v = Value::fifo();
        assert_eq!(v.type_name(), "Fifo");
    }
    #[test]
    fn test_create_operator_type() {
        let v = Value::operator(0, Value::from(42 as i64).unwrap(), Value::none());
        assert_eq!(v.type_name(), "Operator");
    }
}
