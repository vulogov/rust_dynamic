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
    #[test]
    fn test_cast_fifo_type() {
        let mut v = Value::fifo();
        v = v.push(Value::from(42 as i64).unwrap());
        let v2 = v.pop().unwrap();
        assert_eq!(v2.cast_int().unwrap(), 42 as i64);
    }
    #[test]
    fn test_cast_lambda_type() {
        let v = Value::to_lambda(vec![Value::from(42 as i64).unwrap()]);
        let l = v.cast_lambda().unwrap();
        let v2 = l[0].cast_int().unwrap();
        assert_eq!(v2, 42 as i64);
    }
    #[test]
    fn test_cast_pair_type() {
        let v = Value::pair(Value::from_int(42 as i64), Value::from_int(43 as i64));
        let l = v.cast_pair().unwrap();
        let v2 = l[0].cast_int().unwrap();
        assert_eq!(v2, 42 as i64);
    }
    #[test]
    fn test_cast_pair_x_type() {
        let v = Value::pair(Value::from_int(42 as i64), Value::from_int(43 as i64));
        let v2 = v.cast_pair_x().unwrap();
        assert_eq!(v2.cast_int().unwrap(), 42 as i64);
    }
    #[test]
    fn test_cast_pair_y_type() {
        let v = Value::pair(Value::from_int(42 as i64), Value::from_int(43 as i64));
        let v2 = v.cast_pair_y().unwrap();
        assert_eq!(v2.cast_int().unwrap(), 43 as i64);
    }
    #[test]
    fn test_cast_dict_type() {
        let val = Value::dict()
                        .set("answer".to_string(), Value::from(42 as i64).unwrap());
        let d = val.cast_dict().unwrap();
        let v2 = d.get("answer").unwrap();
        assert_eq!(v2.cast_int().unwrap(), 42 as i64);
    }
    #[test]
    fn test_cast_metrics_type() {
        let mut val = Value::metrics_n(1);
        val = val.push(Value::from_float(42.0 as f64));
        let m = val.cast_metrics().unwrap();
        assert_eq!(m[0].data, 42 as f64);
    }
    #[test]
    fn test_cast_operator_opcode() {
        let v = Value::operator(42, Value::from(42 as i64).unwrap());
        assert_eq!(v.cast_operator_opcode().unwrap(), 42 as i32);
    }
    #[test]
    fn test_cast_operator_opvalue() {
        let v = Value::operator(42, Value::from(42 as i64).unwrap());
        let opval = v.cast_operator_value().unwrap();
        assert_eq!(opval.cast_int().unwrap(), 42 as i64);
    }
}
