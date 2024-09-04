#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_pop_list1() {
        let mut v = Value::list();
        v = v.push(Value::from(42.0).unwrap());
        assert_ne!(v.pop(), None);
    }
    #[test]
    fn test_pop_list2() {
        let mut v = Value::list();
        v = v.push(Value::from(42.0).unwrap());
        let v1 = v.pop().unwrap();
        assert_eq!(v1.cast_float().unwrap(), 42.0);
    }
}
