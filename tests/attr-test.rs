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
}
