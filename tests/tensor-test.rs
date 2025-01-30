#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_dynamic::error::BundError;
    use neurons::tensor::Data;

    #[test]
    fn test_create_float_tensor() {
        let mut v = Value::from(42.0).unwrap();
        let t = v.tensor().unwrap();
        match t.data {
            Data::Single(data) => {
                assert_eq!(data[0], 42.0);
            }
            _ => todo!(),
        }
    }
    #[test]
    fn test_create_int_tensor() {
        let mut v = Value::from(42).unwrap();
        let t = v.tensor().unwrap();
        match t.data {
            Data::Single(data) => {
                assert_eq!(data[0], 42.0);
            }
            _ => todo!(),
        }
    }
    #[test]
    fn test_create_float32_tensor() {
        let mut v = Value::from(42.0 as f32).unwrap();
        let t = v.tensor().unwrap();
        match t.data {
            Data::Single(data) => {
                assert_eq!(data[0], 42.0);
            }
            _ => todo!(),
        }
    }
    #[test]
    fn test_create_string_tensor() {
        let mut v = Value::from("Hello").unwrap();
        let t = v.tensor().unwrap();
        match t.data {
            Data::Double(data) => {
                assert_eq!(data[0][0], 72.0);
            }
            _ => todo!(),
        }
    }
}
