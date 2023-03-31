#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use num::complex::Complex;
    use rust_dynamic::value::Value;

    #[test]
    fn test_metrics_create() {
        let v = Value::metrics();
        assert_eq!(v.len(), 128);
    }
    #[test]
    fn test_metrics_push() {
        let mut v = Value::metrics();
        v.push(Value::from(42.0 as f64).unwrap());
        assert_eq!(v.len(), 128);
    }
    #[test]
    fn test_metrics_iter() {
        let mut c: f64 = 0.0;
        let mut v = Value::metrics();
        v = v.push(Value::from(42.0 as f64).unwrap());
        for f in v {
            c += f.get("value").unwrap().cast_float().unwrap();
        }
        assert_eq!(c, 42.0 as f64);
    }
    #[test]
    fn test_export_metrics() {
        let mut val = Value::metrics_n(1);
        val = val.push(Value::from_float(42.0 as f64));
        let m = val.export_float().unwrap();
        assert_eq!(m[0], 42 as f64);
    }
}
