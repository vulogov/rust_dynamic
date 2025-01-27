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
    fn test_car_matrix() {
        let mut m = Value::matrix();
        let v = Value::list()
                .push(Value::from(1.0 as f64).unwrap())
                .push(Value::from(41.0 as f64).unwrap());
        m = m.push(v.clone());
        let matrix_car = m.car().unwrap();
        assert_eq!(matrix_car.car().expect("expecting value").cast_float().unwrap(), 1.0 as f64);
    }
    #[test]
    fn test_cdr_matrix() {
        let mut m = Value::matrix();
        let v0 = Value::list()
                .push(Value::from(1.0 as f64).unwrap())
                .push(Value::from(41.0 as f64).unwrap());
        let v1 = Value::list()
                .push(Value::from(2.0 as f64).unwrap())
                .push(Value::from(41.0 as f64).unwrap());
        m = m.push(v0.clone());
        m = m.push(v1.clone());
        let matrix_cdr = m.cdr().unwrap();
        let matrix_val = matrix_cdr.cast_matrix().unwrap();
        assert_eq!(matrix_val[0][0].cast_float().unwrap(), 2.0 as f64);
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
    #[test]
    fn test_at_list() {
        let v = Value::from(vec![Value::from_int(1 as i64), Value::from_int(2 as i64), Value::from_int(3 as i64)])
                .unwrap()
                .at(1).expect("expecting cdr");
        assert_eq!(v.car().expect("expecting value").cast_int().unwrap(), 2 as i64);
    }
    #[test]
    fn test_head1_list() {
        let v = Value::from(vec![Value::from_int(1 as i64), Value::from_int(2 as i64), Value::from_int(3 as i64)])
                .unwrap()
                .head(2).expect("expecting cdr");
        assert_eq!(v.len(), 2);
    }
    #[test]
    fn test_tail1_list() {
        let v = Value::from(vec![Value::from_int(1 as i64), Value::from_int(2 as i64), Value::from_int(3 as i64)])
                .unwrap()
                .head(3).expect("expecting cdr");
        assert_eq!(v.len(), 3);
    }
    #[test]
    fn test_metrics_car() {
        let mut val = Value::metrics_n(3);
        val = val.push(Value::from_float(42.0 as f64));
        val = val.push(Value::from_float(43.0 as f64));
        val = val.push(Value::from_float(44.0 as f64));
        let m = val.car().expect("expecting car");
        assert_eq!(m.get("value").unwrap().cast_float().unwrap(), 42.0 as f64);
    }
    #[test]
    fn test_metrics_last() {
        let mut val = Value::metrics_n(3);
        val = val.push(Value::from_float(42.0 as f64));
        val = val.push(Value::from_float(43.0 as f64));
        val = val.push(Value::from_float(44.0 as f64));
        let m = val.last().expect("expecting last");
        assert_eq!(m.get("value").unwrap().cast_float().unwrap(), 44.0 as f64);
    }
    #[test]
    fn test_metrics_cdr() {
        let mut val = Value::metrics_n(3);
        val = val.push(Value::from_float(42.0 as f64));
        val = val.push(Value::from_float(43.0 as f64));
        val = val.push(Value::from_float(44.0 as f64));
        val = val.cdr().expect("expecting cdr");
        let m = val.car().expect("expecting car");
        assert_eq!(m.get("value").unwrap().cast_float().unwrap(), 43.0 as f64);
    }
    #[test]
    fn test_queue_car() {
        let mut val = Value::queue();
        val = val.push(Value::from_float(42.0 as f64));
        val = val.push(Value::from_float(43.0 as f64));
        val = val.push(Value::from_float(44.0 as f64));
        let m = val.car().expect("expecting car");
        assert_eq!(m.cast_float().unwrap(), 42.0 as f64);
    }
    #[test]
    fn test_fifo_car() {
        let mut val = Value::fifo();
        val = val.push(Value::from_float(42.0 as f64));
        val = val.push(Value::from_float(43.0 as f64));
        val = val.push(Value::from_float(44.0 as f64));
        let m = val.car().expect("expecting car");
        assert_eq!(m.cast_float().unwrap(), 44.0 as f64);
    }
}
