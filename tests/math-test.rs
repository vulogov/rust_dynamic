#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_math_add() {
        let mut x = Value::from(1.0 as f64).unwrap();
        let y = Value::from(41.0 as f64).unwrap();
        x = x + y;
        assert_eq!(x.cast_float().unwrap(), 42.0);
    }
    #[test]
    fn test_math_add_with_q() {
        let mut x = Value::from(1.0 as f64).unwrap()
                    .set_q(50.0).clone();
        let y = Value::from(41.0 as f64).unwrap();
        x = x + y;
        assert_eq!(x.get_q(), 75.0);
    }
    #[test]
    fn test_math_sub() {
        let mut x = Value::from(43.0 as f64).unwrap();
        let y = Value::from(1.0 as f64).unwrap();
        x = x - y;
        assert_eq!(x.cast_float().unwrap(), 42.0);
    }
    #[test]
    fn test_math_mul() {
        let mut x = Value::from(21.0 as f64).unwrap();
        let y = Value::from(2 as i64).unwrap();
        x = x * y;
        assert_eq!(x.cast_float().unwrap(), 42.0);
    }
    #[test]
    fn test_math_div() {
        let mut x = Value::from(84 as i64).unwrap();
        let y = Value::from(2 as i64).unwrap();
        x = x / y;
        assert_eq!(x.cast_int().unwrap(), 42);
    }
    #[test]
    fn test_string_add() {
        let mut x = Value::from("Hello").unwrap();
        let y = Value::from(" world").unwrap();
        x = x + y;
        assert_eq!(x.cast_string().unwrap(), "Hello world");
    }
    #[test]
    fn test_string_mul() {
        let mut x = Value::from("Hello").unwrap();
        let y = Value::from(2 as i64).unwrap();
        x = x * y;
        assert_eq!(x.cast_string().unwrap(), "HelloHello");
    }
    #[test]
    fn test_math_add_list1() {
        let mut x = Value::list();
        x = x.push(Value::from(41.0 as f64).unwrap())
             .push(Value::from(43.0 as f64).unwrap());
        let y = Value::from(43.0 as f64).unwrap();
        x = x + y;
        assert_eq!(x.len(), 3);
    }
    #[test]
    fn test_math_add_list2() {
        let mut x = Value::list();
        x = x.push(Value::from(41.0 as f64).unwrap())
             .push(Value::from(43.0 as f64).unwrap());
        let mut y = Value::list();
        y = y.push(Value::from(43.0 as f64).unwrap());
        x = x + y;
        assert_eq!(x.len(), 3);
    }
    #[test]
    fn test_add_matrix1() {
        let mut m1 = Value::matrix();
        let mut v1 = Value::list();
        v1 = v1.push(Value::from(42.0).unwrap());
        v1 = v1.push(Value::from(41.0).unwrap());
        m1 = m1.push(v1);
        let mut m2 = Value::matrix();
        let mut v2 = Value::list();
        v2 = v2.push(Value::from(43.0).unwrap());
        v2 = v2.push(Value::from(44.0).unwrap());
        m2 = m2.push(v2);
        m1 = m1 + m2;
        assert_eq!(m1.len(), 2);
    }
    #[test]
    fn test_add_matrix3() {
        let mut m1 = Value::matrix();
        let mut v1 = Value::list();
        v1 = v1.push(Value::from(42.0).unwrap());
        v1 = v1.push(Value::from(41.0).unwrap());
        m1 = m1.push(v1);
        let mut m2 = Value::matrix();
        let mut v2 = Value::list();
        v2 = v2.push(Value::from(43.0).unwrap());
        v2 = v2.push(Value::from(44.0).unwrap());
        m2 = m2.push(v2);
        m1 = m1 + m2;
        let m1_vec = m1.cast_matrix().unwrap();
        assert_eq!(m1_vec[0][0].cast_float().unwrap(), 85.0 as f64);
    }
    #[test]
    fn test_add_matrix2() {
        let mut m1 = Value::matrix();
        let mut v1 = Value::list();
        v1 = v1.push(Value::from(42.0).unwrap());
        v1 = v1.push(Value::from(41.0).unwrap());
        m1 = m1.push(v1);
        let mut v2 = Value::list();
        v2 = v2.push(Value::from(43.0).unwrap());
        v2 = v2.push(Value::from(44.0).unwrap());
        m1 = m1 + v2;
        assert_eq!(m1.len(), 4);
    }
    #[test]
    fn test_textbuffer_add_string() {
        let mut x = Value::text_buffer("Hello".to_string());
        let y = Value::from(" world").unwrap();
        x = x + y;
        assert_eq!(x.cast_string().unwrap(), "Hello world");
    }
    #[test]
    fn test_binary_add_string() {
        use rust_dynamic::types::STRING;
        let mut x = Value::binary();
        x = x.push(Value::from_string("Hello "));
        let y = Value::from("world").unwrap();
        x = x + y;
        assert_eq!(x.conv(STRING).unwrap().cast_string().unwrap(), "Hello world");
    }

    #[test]
    fn test_textbuffer_add_float() {
        let mut x = Value::text_buffer("Hello ".to_string());
        let y = Value::from(3.14).unwrap();
        x = x + y;
        assert_eq!(x.cast_string().unwrap(), "Hello 3.14");
    }
    #[test]
    fn test_math_json_merge() {
        let mut x = Value::json(serde_json::json!([1]));
        let y = Value::json(serde_json::json!([2]));
        x = x + y;
        println!("{:?}", x.cast_json());
        assert_eq!(x.cast_json().unwrap().as_array().unwrap().len(), 2);
    }
    #[test]
    fn test_math_metrics_push_through_add() {
        let mut x = Value::metrics();
        for n in 1..129 {
            x = x + Value::from(n as f64).unwrap();
        }
        let m = x.car().expect("expecting car");
        assert_eq!(m.get("value").unwrap().cast_float().unwrap(), 1.0 as f64);
    }
}
