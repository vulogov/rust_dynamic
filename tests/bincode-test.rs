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
        assert_eq!(bin_data.len(), 63);
    }
    #[test]
    fn test_serialize_unwrap_int() {
        let data = Value::from(42 as i64).unwrap();
        let bin_data = data.wrap().unwrap();
        let data2 = bin_data.unwrap().unwrap();
        assert_eq!(data2.cast_int().unwrap(), 42);
    }
    #[test]
    fn test_serialize_unwrap_dict() {
        let data = Value::dict();
        let bin_data = data.wrap().unwrap();
        let data2 = bin_data.unwrap().unwrap();
        assert_eq!(data2.len(), 0);
    }
}
