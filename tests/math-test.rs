#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_math_add() {
        let mut x = Value::from(1.0 as f64).unwrap();
        let y = Value::from(41.0 as f64).unwrap();
        x = x + y;
        assert_eq!(x.cast_float().unwrap(), 42.0);
    }
    #[test]
    fn test_math_add_with_q() {
        let mut x = Value::from(1.0 as f64).unwrap()
                    .set_q(50.0).clone();
        let y = Value::from(41.0 as f64).unwrap();
        x = x + y;
        assert_eq!(x.get_q(), 75.0);
    }
    #[test]
    fn test_math_sub() {
        let mut x = Value::from(43.0 as f64).unwrap();
        let y = Value::from(1.0 as f64).unwrap();
        x = x - y;
        assert_eq!(x.cast_float().unwrap(), 42.0);
    }
    #[test]
    fn test_math_mul() {
        let mut x = Value::from(21.0 as f64).unwrap();
        let y = Value::from(2 as i64).unwrap();
        x = x * y;
        assert_eq!(x.cast_float().unwrap(), 42.0);
    }
    #[test]
    fn test_math_div() {
        let mut x = Value::from(84 as i64).unwrap();
        let y = Value::from(2 as i64).unwrap();
        x = x / y;
        assert_eq!(x.cast_int().unwrap(), 42);
    }
    #[test]
    fn test_string_add() {
        let mut x = Value::from("Hello").unwrap();
        let y = Value::from(" world").unwrap();
        x = x + y;
        assert_eq!(x.cast_string().unwrap(), "Hello world");
    }
    #[test]
    fn test_string_mul() {
        let mut x = Value::from("Hello").unwrap();
        let y = Value::from(2 as i64).unwrap();
        x = x * y;
        assert_eq!(x.cast_string().unwrap(), "HelloHello");
    }
}
