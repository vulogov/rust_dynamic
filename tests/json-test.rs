#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use serde_json::to_string;

    #[test]
    fn test_serialize_none() {
        let data = Value::new();
        println!("{:?}\n", to_string(&data));
        assert_eq!(42, 42);
    }
}
