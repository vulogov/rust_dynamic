#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_cast_float_type() {
        let v = Value::from(42.0).unwrap();
        assert_eq!(v.cast_float().unwrap(), 42.0);
    }
    #[test]
    fn test_cast_int_type() {
        let v = Value::from(42 as i64).unwrap();
        assert_eq!(v.cast_int().unwrap(), 42 as i64);
    }
    #[test]
    fn test_cast_bool_type() {
        let v = Value::from(true).unwrap();
        assert_eq!(v.cast_bool().unwrap(), true);
    }
    #[test]
    fn test_cast_string_type() {
        let v = Value::from("Hello world").unwrap();
        assert_eq!(v.cast_string().unwrap(), "Hello world");
    }
    #[test]
    fn test_cast_list_type() {
        let v = Value::from_list(vec![Value::from(42 as i64).unwrap()]);
        let l = v.cast_list().unwrap();
        let v2 = l[0].cast_int().unwrap();
        assert_eq!(v2, 42 as i64);
    }
}
