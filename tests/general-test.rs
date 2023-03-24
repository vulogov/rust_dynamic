#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_id_1() {
        let v = Value::new();
        assert_ne!(v.id.len(), 0);
    }
    #[test]
    fn test_hash_1() {
        use std::collections::HashMap;
        let key = Value::from(42.0 as f64).unwrap();
        let mut h: HashMap<Value, String> = HashMap::new();
        h.insert(key, "value".to_string());
        assert_eq!(h.len(), 1);
    }
    #[test]
    fn test_dup_duplicate() {
        let val1 = Value::from(42.0 as f64).unwrap();
        let val2 = val1.dup().unwrap();
        assert_eq!(val1.cast_float().unwrap(), val2.cast_float().unwrap());
    }
    #[test]
    fn test_dup_duplicate_id_different() {
        let val1 = Value::from(42.0 as f64).unwrap();
        let val2 = val1.dup().unwrap();
        assert_ne!(val1.id, val2.id);
    }
}
