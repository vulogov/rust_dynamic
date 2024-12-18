#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_iter_non_list() {
        let mut c = 0.0;
        let v = Value::from(42.0 as f64).unwrap();
        for i in v {
            c += i.cast_float().unwrap();
        }
        assert_eq!(c, 42.0);
    }
    #[test]
    fn test_iter_list() {
        let mut c = 0.0;
        let v = Value::list()
                .push(Value::from(1.0 as f64).unwrap())
                .push(Value::from(41.0 as f64).unwrap());
        for i in v {
            c += i.cast_float().unwrap();
        }
        assert_eq!(c, 42.0);
    }
    #[test]
    fn test_iter_matrix() {
        let mut c: usize = 0;
        let mut m = Value::matrix();
        let v = Value::list()
                .push(Value::from(1.0 as f64).unwrap())
                .push(Value::from(41.0 as f64).unwrap());
        m = m.push(v.clone());
        for i in m {
            c += i.cast_list().unwrap().len();
        }
        assert_eq!(c, 2);
    }
}
