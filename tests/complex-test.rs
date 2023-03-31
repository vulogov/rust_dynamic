#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use num::complex::Complex;
    use rust_dynamic::value::Value;

    #[test]
    fn test_complex_int_type() {
        let v = Value::from(Complex::new(10 as i64, 20 as i64)).unwrap();
        assert_eq!(v.type_name(), "ComplexInteger");
    }
    #[test]
    fn test_complex_float_type() {
        let v = Value::from(Complex::new(10.0 as f64, 20.0 as f64)).unwrap();
        assert_eq!(v.type_name(), "ComplexFloat");
    }
    #[test]
    fn test_complex_int_cast() {
        let v = Value::from(Complex::new(10.0 as i64, 20.0 as i64)).unwrap();
        assert_eq!(v.cast_complex_int().unwrap(), Complex::new(10.0 as i64, 20.0 as i64));
    }
    #[test]
    fn test_complex_float_cast() {
        let v = Value::from(Complex::new(10.0 as f64, 20.0 as f64)).unwrap();
        assert_eq!(v.cast_complex_float().unwrap(), Complex::new(10.0 as f64, 20.0 as f64));
    }
    #[test]
    fn test_complex_int_eq() {
        let v1 = Value::from(Complex::new(10.0 as i64, 20.0 as i64)).unwrap();
        let v2 = Value::from(Complex::new(10.0 as i64, 20.0 as i64)).unwrap();
        assert!(v1 == v2);
    }
    #[test]
    fn test_complex_float_eq() {
        let v1 = Value::from(Complex::new(10.0 as f64, 20.0 as f64)).unwrap();
        let v2 = Value::from(Complex::new(10.0 as f64, 20.0 as f64)).unwrap();
        assert!(v1 == v2);
    }
}
