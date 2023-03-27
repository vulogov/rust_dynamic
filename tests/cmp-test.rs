#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_cmp_float() {
        let v1 = Value::from(42.0).unwrap();
        let v2 = Value::from(42.0).unwrap();
        assert!(v1 == v2);
    }
    #[test]
    fn test_cmp_int() {
        let v1 = Value::from(42 as i64).unwrap();
        let v2 = Value::from(42 as i64).unwrap();
        assert!(v1 == v2);
    }
    #[test]
    fn test_cmp_gt_int() {
        let v1 = Value::from(42 as i64).unwrap();
        let v2 = Value::from(21 as i64).unwrap();
        assert!(v1 > v2);
    }
    #[test]
    fn test_cmp_str() {
        let v1 = Value::from("HelloHello").unwrap();
        let v2 = Value::from("HelloHello").unwrap();
        assert!(v1 == v2);
    }
}
