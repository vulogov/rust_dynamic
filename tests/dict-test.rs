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
    fn test_has_key() {
        let val = Value::dict()
                        .set("answer".to_string(), Value::from(42 as i64).unwrap());
        let res = val.has_key("answer").unwrap().cast_bool().unwrap();
        assert_eq!(res, true);
    }
    #[test]
    fn test_valuemap_has_key() {
        let val = Value::valuemap()
                        .set_vmap(Value::from_string("answer"), Value::from(42 as i64).unwrap());
        let res = val.cast_valuemap().unwrap();
        assert_eq!(res.values().nth(0).unwrap().cast_int().unwrap(), 42 as i64);
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

    #[test]
    fn test_message_get() {
        let v = Value::message(Value::from_string("ABC"), Value::from_string("CDE"), Value::from_string("GHI"));
        let val = v.get("from".to_string()).unwrap();
        assert_eq!(val.cast_string().unwrap(), "ABC".to_string());
    }

    #[test]
    fn test_dict_set_conditional() {
        let val = Value::conditional()
                        .set("try".to_string(), Value::from(42 as i64).unwrap());
        let val2 = val.get("try".to_string()).unwrap();
        assert_eq!(val2.cast_int().unwrap(), 42 as i64);
    }
}
