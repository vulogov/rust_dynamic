#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_serialize_int() {
        let data = Value::from(42 as i64).unwrap();
        let json_out = data.to_json().unwrap();
        assert_ne!(json_out.len(), 0);
    }
    #[test]
    fn test_serialize_deserialize_int() {
        let data = Value::from(42 as i64).unwrap();
        let json_out = data.to_json().unwrap();
        let data2 = Value::from_json(json_out).unwrap();
        assert_eq!(data2.cast_int().unwrap(), 42 as i64);
    }
    #[test]
    fn test_serialize_dict() {
        let data = Value::dict();
        let json_out = data.to_json().unwrap();
        assert_ne!(json_out.len(), 0);
    }
    #[test]
    fn test_as_json_value_int() {
        use serde_json;
        let v = Value::from_int(42 as i64).as_json_value();
        assert_eq!(serde_json::from_value::<i64>(v).unwrap(), 42 as i64);
    }
    #[test]
    fn test_as_json_value_string() {
        use serde_json;
        let v = Value::from_string("hello").as_json_value();
        assert_eq!(serde_json::from_value::<String>(v).unwrap(), "hello".to_string());
    }
    #[test]
    fn test_as_json_value_list() {
        use serde_json;
        let v = Value::from_list(vec![Value::from_int(1), Value::from_int(2)]).as_json_value();
        assert_eq!(v.as_array().unwrap().len(), 2);
    }
    #[test]
    fn test_as_json_value_dict() {
        use serde_json;
        let val = Value::dict()
                        .set("answer".to_string(), Value::from(42 as i64).unwrap())
                        .as_json_value();
        assert_eq!(val.as_object().unwrap().len(), 1);
    }
}
