#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_dynamic::types::*;

    #[test]
    fn test_set_tag_string() {
        let mut v = Value::from(41.0).unwrap();
        v.set_tag("prefix".to_string(), "prefix".to_string());
        assert_eq!(v.get_tag("prefix".to_string()).unwrap(), "prefix");
    }

    #[test]
    fn test_set_tag_str() {
        let mut v = Value::from(41.0).unwrap();
        v.set_tag("prefix", "prefix");
        assert_eq!(v.get_tag("prefix").unwrap(), "prefix");
    }

    #[test]
    fn test_has_tag_str() {
        let mut v = Value::from(41.0).unwrap();
        v.set_tag("prefix", "prefix");
        assert!(v.has_tag("prefix"));
    }

    #[test]
    fn test_serialize_tag() {
        let mut data = Value::from(42 as i64).unwrap();
        data.set_tag("prefix", "prefix");
        let bin_data = data.wrap().unwrap();
        let mut data2 = bin_data.unwrap().unwrap();
        assert_eq!(data2.get_tag("prefix").unwrap(), "prefix");
    }
}
