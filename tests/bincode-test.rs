#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_serialize_bin_int() {
        let mut data = Value::from(42 as i64).unwrap();
        let bin_out = data.to_binary().unwrap();
        assert_ne!(bin_out.len(), 0);
    }
    #[test]
    fn test_serialize_deserialize_bin_int() {
        let mut data = Value::from(42 as i64).unwrap();
        let bin_out = data.to_binary().unwrap();
        let mut data2 = Value::from_binary(bin_out).unwrap();
        assert_eq!(data2.cast_int().unwrap(), 42 as i64);
    }
}
