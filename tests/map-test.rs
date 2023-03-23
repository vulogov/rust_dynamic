#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_map_float() {
        let mut v = Value::from(42.0).unwrap();
        v = v.map_float(f64::sin);
        assert_eq!(v.cast_float().unwrap(), f64::sin(42.0));
    }
}
