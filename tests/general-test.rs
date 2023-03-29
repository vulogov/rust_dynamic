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
    #[test]
    fn test_elapsed() {
        use std::{thread, time};
        let ten_millis = time::Duration::from_millis(10);
        let val = Value::now();
        thread::sleep(ten_millis);
        assert_ne!(val.elapsed().unwrap(), 0);
    }
    #[test]
    fn test_time_compare_nanos() {
        let val = Value::now();
        let dt = val.get_time_as_datetime().unwrap();
        assert_eq!(dt.timestamp_nanos() as u128, val.cast_timestamp().unwrap());
    }
    #[test]
    fn test_time_compare_times() {
        let val = Value::now();
        assert!(val == val);
    }
    #[test]
    fn test_time_gt_compare_times() {
        use std::{thread, time};
        let ten_millis = time::Duration::from_millis(10);
        let val1 = Value::now();
        thread::sleep(ten_millis);
        let val2 = Value::now();
        assert!(val2 > val1);
    }
}
