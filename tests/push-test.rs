#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_push_regular() {
        let mut v = Value::from(41.0).unwrap();
        v = v.push(Value::from(42.0).unwrap());
        assert_eq!(v.cast_float().unwrap(), 42.0);
    }
    #[test]
    fn test_push_list() {
        let mut v = Value::list();
        v = v.push(Value::from(42.0).unwrap());
        assert_eq!(v.len(), 1);
    }
    #[test]
    fn test_push_list_json1() {
        let mut v = Value::json(serde_json::json!([]));
        v = v.push(Value::from(42.0).unwrap());
        let j_value = v.cast_json().unwrap();
        assert_eq!(j_value.as_array().unwrap().len(), 1);
    }
    #[test]
    fn test_push_list_json2() {
        let mut v = Value::json(serde_json::json!([]));
        v = v.push(Value::from(42.0).unwrap());
        let j_value = v.cast_json().unwrap();
        assert_eq!(j_value.as_array().unwrap()[0].as_f64().unwrap(), 42.0);
    }
    #[test]
    fn test_push_lambda() {
        let mut v = Value::lambda();
        v = v.push(Value::from(42.0).unwrap());
        assert_eq!(v.len(), 1);
    }
    #[test]
    fn test_push_queue_len() {
        let v = Value::queue()
            .push(Value::from_int(42))
            .push(Value::from_int(41));
        assert_eq!(v.len(), 2);
    }
    #[test]
    fn test_push_queue_val() {
        let v = Value::queue()
            .push(Value::from_int(42))
            .push(Value::from_int(41));
        let val = v.cast_queue().unwrap();
        assert_eq!(val.cast_int().unwrap(), 42);
    }
    #[test]
    fn test_push_fifo_len() {
        let v = Value::fifo()
            .push(Value::from_int(41))
            .push(Value::from_int(42));
        assert_eq!(v.len(), 2);
    }
    #[test]
    fn test_push_fifo_val() {
        let v = Value::fifo()
            .push(Value::from_int(41))
            .push(Value::from_int(42));
        let val = v.cast_fifo().unwrap();
        assert_eq!(val.cast_int().unwrap(), 42);
    }
    #[test]
    fn test_push_queue_push_pull() {
        let mut v = Value::queue()
            .push(Value::from_int(41))
            .push(Value::from_int(42))
            .push(Value::from_int(43))
            .push(Value::from_int(44));
        v = v.pull();
        let val = v.cast_queue().unwrap();
        assert_eq!(val.cast_int().unwrap(), 42);
    }
    #[test]
    fn test_push_fifo_push_pull() {
        let mut v = Value::fifo()
            .push(Value::from_int(40))
            .push(Value::from_int(41))
            .push(Value::from_int(42))
            .push(Value::from_int(43));
        v = v.pull();
        let val = v.cast_fifo().unwrap();
        assert_eq!(val.cast_int().unwrap(), 42);
    }
    #[test]
    fn test_push_list_push_pull() {
        let mut v = Value::list()
            .push(Value::from_int(40))
            .push(Value::from_int(41))
            .push(Value::from_int(42))
            .push(Value::from_int(43));
        v = v.pull();
        let val = v.cast_list().unwrap();
        assert_eq!(val[val.len()-1].cast_int().unwrap(), 42);
    }

    #[test]
    fn test_push_list_push_inplace() {
        let mut v = Value::list();
        v.push_inplace(Value::from_int(42)).unwrap();
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn test_push_textbuffer() {
        let mut v = Value::text_buffer("Answer is ".to_string());
        v = v.push(Value::from_int(42));
        assert_eq!(v.cast_string().unwrap(),  "Answer is 42");
    }
}
