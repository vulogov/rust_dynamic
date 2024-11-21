#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_dynamic::types::*;

    #[test]
    fn test_conv_inner_list() {
        let mut v1 = Value::list();
        v1 = v1.push(Value::from("42.0").unwrap());
        v1 = v1.push(Value::from("41.0").unwrap());
        let val = v1.conv_inner(FLOAT).unwrap();
        let list_val = val.cast_list().unwrap();
        assert_eq!(list_val[0].cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_conv_inner_matrix() {
        let mut m1 = Value::matrix();
        let mut v1 = Value::list();
        v1 = v1.push(Value::from("42.0").unwrap());
        v1 = v1.push(Value::from("41.0").unwrap());
        m1 = m1.push(v1);
        let val = m1.conv_inner(FLOAT).unwrap();
        let mat_val = val.cast_matrix().unwrap();
        assert_eq!(mat_val[0][0].cast_float().unwrap(), 42.0 as f64);
    }
}
