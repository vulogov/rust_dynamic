#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_dynamic::types::*;

    #[test]
    fn test_reduce_float() {
        let mut v = Value::from(41.0).unwrap();
        v = v.freduce(|x: Value,y: Value| -> Value { x+y }, Value::from_float(1.0 as f64));
        assert_eq!(v.cast_float().unwrap(), 42.0 as f64);
    }
    #[test]
    fn test_reduce_list_float() {
        let mut v = Value::from_list(vec![Value::from_float(41.0 as f64), Value::from_float(1.0 as f64)]);
        v = v.freduce(|x: Value,y: Value| -> Value { x+y }, Value::from_float(0.0 as f64));
        assert_eq!(v.cast_float().unwrap(), 42.0 as f64);
    }
}
