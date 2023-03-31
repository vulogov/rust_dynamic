#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::{Value, Applicative};
    use rust_dynamic::types::*;


    #[test]
    fn test_applicative() {
        fn comp_sin(value: Value) -> Value {
            match value.data {
                Val::F64(f_val) => {
                    return Value::from_float(f64::sin(f_val));
                }
                _ => return value,
            }
        }
        let sin = Applicative::new(comp_sin);
        let res = sin.apply(Value::from(42.0).unwrap());
        assert_eq!(res.cast_float().unwrap(), f64::sin(42.0));
    }
}
