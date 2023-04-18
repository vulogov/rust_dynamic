#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_push_regular() {
        let mut v = Value::from(41.0).unwrap();
        v = v.push(Value::from(42.0).unwrap());
        assert_eq!(v.cast_float().unwrap(), 42.0);
    }
    #[test]
    fn test_push_list() {
        let mut v = Value::list();
        v = v.push(Value::from(42.0).unwrap());
        assert_eq!(v.len(), 1);
    }
    #[test]
    fn test_push_lambda() {
        let mut v = Value::lambda();
        v = v.push(Value::from(42.0).unwrap());
        assert_eq!(v.len(), 1);
    }
}
