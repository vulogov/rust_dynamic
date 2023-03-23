#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_len_list() {
        let v = Value::from(vec![Value::new(), Value::new()]).unwrap();
        assert_eq!(v.len(), 2);
    }
    #[test]
    fn test_len_nodata() {
        let v = Value::nodata();
        assert_eq!(v.len(), 0);
    }
    #[test]
    fn test_len_regular() {
        let v = Value::from(42 as i64).unwrap();
        assert_eq!(v.len(), 1);
    }
}
