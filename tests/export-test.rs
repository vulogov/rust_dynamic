#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_export_float_val() {
        let v = Value::from(42.0).unwrap();
        let res = v.export_float().unwrap();
        assert_eq!(res[0], 42.0 as f64);
    }
    #[test]
    fn test_export_float_val_from_int() {
        let v = Value::from(42 as i64).unwrap();
        let res = v.export_float().unwrap();
        assert_eq!(res[0], 42.0 as f64);
    }
    #[test]
    fn test_export_float_list() {
        let res = Value::list()
                .push(Value::from(1.0 as f64).unwrap())
                .push(Value::from(41.0 as f64).unwrap())
                .export_float().unwrap();
        assert_eq!(res[1], 41.0);
    }
    #[test]
    fn test_export_float_list_from_int() {
        let res = Value::list()
                .push(Value::from(1 as i64).unwrap())
                .push(Value::from(41 as i64).unwrap())
                .export_float().unwrap();
        assert_eq!(res[1], 41.0);
    }
}
