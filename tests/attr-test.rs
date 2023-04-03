#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_attr_attr() {
        let v = Value::from(42 as i64).unwrap()
                    .attr(vec![Value::none()]);
        assert_eq!(v.attr_len(), 1);
    }
    #[test]
    fn test_attr_attr_add() {
        let v = Value::from(42 as i64).unwrap()
                    .attr(vec![Value::none()])
                    .attr_add(Value::none());
        assert_eq!(v.attr_len(), 2);
    }
    #[test]
    fn test_attr_attr_value_check() {
        let v = Value::from(42 as i64).unwrap()
                    .attr(vec![Value::from(42.0 as f64).unwrap()]);
        assert_eq!(v.attr[0].cast_float().unwrap(), 42.0 as f64);
    }
    #[test]
    fn test_attr_attr_merge_check_len() {
        let v = Value::from(42 as i64).unwrap()
                    .attr(vec![Value::from(42.0 as f64).unwrap()])
                    .attr_merge(vec![Value::from(42.0 as f64).unwrap()]);
        assert_eq!(v.attr_len(), 2);
    }
    #[test]
    fn test_attr_attr_merge_check_value() {
        let v = Value::from(42 as i64).unwrap()
                    .attr(vec![Value::from(41.0 as f64).unwrap()])
                    .attr_merge(vec![Value::from(42.0 as f64).unwrap()]);
        assert_eq!(v.attr[0].cast_float().unwrap(), 42.0 as f64);
    }
    #[test]
    fn test_attr_attr_merge_check_if_attr_is_wrappable() {
        let v = Value::from(42 as i64).unwrap()
                    .attr(vec![Value::from(41.0 as f64).unwrap()])
                    .attr_merge(vec![Value::from(42.0 as f64).unwrap()]);
        let bin_data = v.wrap().unwrap();
        let v2 = bin_data.unwrap().unwrap();
        assert_eq!(v2.attr[0].cast_float().unwrap(), 42.0 as f64);
    }
}
