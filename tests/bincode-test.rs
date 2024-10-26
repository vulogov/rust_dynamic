#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_serialize_bin_int() {
        let data = Value::from(42 as i64).unwrap();
        let bin_out = data.to_binary().unwrap();
        assert_ne!(bin_out.len(), 0);
    }
    #[test]
    fn test_serialize_deserialize_bin_int() {
        let data = Value::from(42 as i64).unwrap();
        let bin_out = data.to_binary().unwrap();
        let data2 = Value::from_binary(bin_out).unwrap();
        assert_eq!(data2.cast_int().unwrap(), 42 as i64);
    }
    #[test]
    fn test_serialize_wrap_int() {
        let data = Value::from(42 as i64).unwrap();
        let bin_data = data.wrap().unwrap();
        assert_eq!(bin_data.len(), 79);
    }
    #[test]
    fn test_serialize_unwrap_int() {
        let data = Value::from(42 as i64).unwrap();
        let bin_data = data.wrap().unwrap();
        let data2 = bin_data.unwrap().unwrap();
        assert_eq!(data2.cast_int().unwrap(), 42);
    }
    #[test]
    fn test_serialize_unwrap_json1() {
        let data = Value::json(serde_json::json!(42));
        let bin_data = data.wrap().unwrap();
        let data2: Value = bin_data.unwrap().unwrap();
        let data3 = data2.cast_json_to_value().unwrap();
        assert_eq!(data3.cast_int().unwrap(), 42);
    }
    #[test]
    fn test_serialize_unwrap_dict() {
        let data = Value::dict();
        let bin_data = data.wrap().unwrap();
        let data2 = bin_data.unwrap().unwrap();
        assert_eq!(data2.len(), 0);
    }
    #[test]
    fn test_serialize_unwrap_list() {
        let data = Value::list();
        let bin_data = data.wrap().unwrap();
        let data2 = bin_data.unwrap().unwrap();
        assert_eq!(data2.len(), 0);
    }
    #[test]
    fn test_serialize_operator1() {
        let data = Value::operator(42, Value::from(42 as i64).unwrap(), Value::none());
        let bin_data = data.wrap().unwrap();
        let data2 = bin_data.unwrap().unwrap();
        assert_eq!(data2.type_name(), "Operator");
    }
    #[test]
    fn test_serialize_operator2() {
        let data = Value::operator(42, Value::from(42 as i64).unwrap(), Value::none());
        let bin_data = data.wrap().unwrap();
        let data2 = bin_data.unwrap().unwrap();
        assert_eq!(data2.cast_operator_opcode().unwrap(), 42 as i32);
    }
    #[test]
    fn test_serialize_operator3() {
        let data = Value::operator(42, Value::from(42 as i64).unwrap(), Value::none());
        let bin_data = data.wrap().unwrap();
        let data2 = bin_data.unwrap().unwrap();
        let opval = data2.cast_operator_value().unwrap();
        assert_eq!(opval.cast_int().unwrap(), 42 as i64);
    }
    #[test]
    fn test_serialize_operator4() {
        let mut data = Value::list();
        let data1 = Value::operator(42, Value::from(42 as i64).unwrap(), Value::none());
        data = data.push(data1);
        let bin_data = data.wrap().unwrap();
        let data2 = bin_data.unwrap().unwrap();
        assert_ne!(data2.len(), 0);
    }
    #[test]
    fn test_serialize_operator5() {
        let mut data = Value::list();
        let data1 = Value::operator(42, Value::from(42 as i64).unwrap(), Value::none());
        data = data.push(data1);
        let bin_data = data.wrap().unwrap();
        let data2 = bin_data.unwrap().unwrap();
        let op_object = data2.cast_list().unwrap();
        let opval = op_object[0].cast_operator_value().unwrap();
        assert_eq!(opval.cast_int().unwrap(), 42 as i64);
    }
    #[test]
    fn test_serialize_lambda() {
        let data = Value::lambda()
                        .push(Value::operator(42, Value::from(42 as i64).unwrap(), Value::none()))
                        .push(Value::operator(43, Value::from(43 as i64).unwrap(), Value::none()));

        let buffer = data.compile().unwrap();
        assert_ne!(buffer.len(), 0);
    }
}
