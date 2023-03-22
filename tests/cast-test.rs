#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_cast_float_type() {
        let mut v = Value::from(42.0).unwrap();
        assert_eq!(v.cast_float().unwrap(), 42.0);
    }
    #[test]
    fn test_cast_int_type() {
        let mut v = Value::from(42 as i64).unwrap();
        assert_eq!(v.cast_int().unwrap(), 42 as i64);
    }
    #[test]
    fn test_cast_bool_type() {
        let mut v = Value::from(true).unwrap();
        assert_eq!(v.cast_bool().unwrap(), true);
    }
    #[test]
    fn test_cast_string_type() {
        let mut v = Value::from("Hello world").unwrap();
        assert_eq!(v.cast_string().unwrap(), "Hello world");
    }
}
