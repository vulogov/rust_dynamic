#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_dict_set() {
        let val = Value::dict()
                        .set("answer".to_string(), Value::from(42 as i64).unwrap());
        assert_eq!(val.len(), 1);
    }

    #[test]
    fn test_dict_set_with_spaces() {
        let val = Value::dict()
                        .set("  answer    ".to_string(), Value::from(42 as i64).unwrap());
        let val2 = val.get("answer".to_string()).unwrap();
        assert_eq!(val2.cast_int().unwrap(), 42 as i64);
    }

    #[test]
    fn test_dict_set_raw_key() {
        let val = Value::dict()
                        .set_key_raw(" answer ".to_string(), Value::from(42 as i64).unwrap());
        let val2 = val.get(" answer ".to_string()).unwrap();
        assert_eq!(val2.cast_int().unwrap(), 42 as i64);
    }

    #[test]
    fn test_association_get() {
        let val = Value::association("answer".to_string(), Value::from(42 as i64).unwrap());
        let val2 = val.get("answer".to_string()).unwrap();
        assert_eq!(val2.cast_int().unwrap(), 42 as i64);
    }
}
