#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_serialize_int() {
        let mut data = Value::from(42 as i64).unwrap();
        let json_out = data.to_json().unwrap();
        assert_ne!(json_out.len(), 0);
    }
    #[test]
    fn test_serialize_deserialize_int() {
        let mut data = Value::from(42 as i64).unwrap();
        let json_out = data.to_json().unwrap();
        let mut data2 = Value::from_json(json_out).unwrap();
        assert_eq!(data2.cast_int().unwrap(), 42 as i64);
    }
}
