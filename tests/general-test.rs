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
}
