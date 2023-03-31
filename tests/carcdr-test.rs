#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_car_list() {
        let v = Value::from(vec![Value::from_int(1 as i64), Value::from_int(2 as i64), Value::from_int(3 as i64)]).unwrap();
        assert_eq!(v.car().expect("expecting value").cast_int().unwrap(), 1 as i64);
    }
    #[test]
    fn test_last_list() {
        let v = Value::from(vec![Value::from_int(1 as i64), Value::from_int(2 as i64), Value::from_int(3 as i64)]).unwrap();
        assert_eq!(v.last().expect("expecting value").cast_int().unwrap(), 3 as i64);
    }
    #[test]
    fn test_cdr_list() {
        let v = Value::from(vec![Value::from_int(1 as i64), Value::from_int(2 as i64), Value::from_int(3 as i64)])
                .unwrap()
                .cdr().expect("expecting cdr");
        assert_eq!(v.car().expect("expecting value").cast_int().unwrap(), 2 as i64);
    }
}
