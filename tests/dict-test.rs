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
}
