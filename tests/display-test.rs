#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::{Value};


    #[test]
    fn test_display_int() {
        let v = Value::from(42 as i64).unwrap();
        assert_eq!(format!("{}", v), "42");
    }
    #[test]
    fn test_display_float() {
        let v = Value::from(42.0 as f64).unwrap();
        assert_eq!(format!("{}", v), "42.0");
    }
    #[test]
    fn test_display_string() {
        let v = Value::from("Hello world").unwrap();
        assert_eq!(format!("{}", v), "Hello world");
    }
    #[test]
    fn test_display_bool() {
        let v = Value::from(true).unwrap();
        assert_eq!(format!("{}", v), "true");
    }
}
