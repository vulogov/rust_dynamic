use std::ops::{Add, Sub, Mul, Div};
use num::complex::Complex;
use crate::value::Value;
use crate::types::*;
extern crate json_value_merge;
use json_value_merge::Merge;

#[derive(Debug, Clone)]
pub enum Ops {
    Add,
    Sub,
    Mul,
    Div,
}

fn numeric_op_metric_number(op: Ops, mut x: Value, y: Value) -> Value {
    match op {
        Ops::Add => {
            return x.push(y)
        }
        _ => x,
    }
}

fn numeric_op_json_json(op: Ops, x: serde_json::Value, y: serde_json::Value) -> serde_json::Value {
    match op {
        Ops::Add => {
            let mut res = x.clone();
            res.merge(&y);
            return res.into();
        }
        _ => x,
    }
}

fn numeric_op_matrix(op: Ops, x: Vec<Vec<Value>>, y: Vec<Vec<Value>>) -> Vec<Vec<Value>> {
    if &x.len() != &y.len() {
        return x;
    }
    let mut res: Vec<Vec<Value>> = Vec::new();

    let mut x_i: usize = 0;
    let mut y_i: usize = 0;

    for x_v in &x {
        if x_v.len() != y[y_i].len() {
            return x;
        }
        let mut row: Vec<Value> = Vec::new();
        for y_v in x_v {
            let x_res = match Value::numeric_op(op.clone(), y_v.clone(), y[y_i][x_i].clone()) {
                Ok(x_res) => x_res,
                Err(_) => {
                    return x;
                }
            };
            row.push(x_res);
            x_i = x_i + 1;
        }
        res.push(row);
        x_i = 0;
        y_i = y_i + 1;
    }
    return res;
}

fn numeric_op_float_float(op: Ops, x: f64, y: f64) -> f64 {
    match op {
        Ops::Add => x + y,
        Ops::Sub => x - y,
        Ops::Mul => x * y,
        Ops::Div => x / y,
    }
}

fn numeric_op_int_int(op: Ops, x: i64, y: i64) -> i64 {
    match op {
        Ops::Add => x + y,
        Ops::Sub => x - y,
        Ops::Mul => x * y,
        Ops::Div => x / y,
    }
}

fn numeric_op_cpx_int_cpx_int(op: Ops, x: Complex<i64>, y: Complex<i64>) -> Complex<i64> {
    match op {
        Ops::Add => x + y,
        Ops::Sub => x - y,
        Ops::Mul => x * y,
        Ops::Div => x / y,
    }
}

fn numeric_op_cpx_float_cpx_float(op: Ops, x: Complex<f64>, y: Complex<f64>) -> Complex<f64> {
    match op {
        Ops::Add => x + y,
        Ops::Sub => x - y,
        Ops::Mul => x * y,
        Ops::Div => x / y,
    }
}

fn string_op_string_string(op: Ops, x: String, y: String) -> String {
    match op {
        Ops::Add => format!("{}{}", &x, &y),
        _ => x,
    }
}

fn string_op_string_int(op: Ops, x: String, y: i64) -> String {
    match op {
        Ops::Mul => x.repeat(y as usize),
        Ops::Add => format!("{}{}", x, y),
        _ => x,
    }
}

fn string_op_string_float(op: Ops, x: String, y: f64) -> String {
    match op {
        Ops::Mul => x.repeat(y as usize),
        Ops::Add => format!("{}{:.}", x, y),
        _ => x,
    }
}

impl Value {
    pub fn numeric_op(op: Ops, mut x: Value, mut y: Value) -> Result<Value, Box<dyn std::error::Error>> {
        match x.data {
            Val::Metrics(ref _m_x) => {
                let y_val = match y.conv(FLOAT) {
                    Ok(y_val) => y_val,
                    Err(_) => {
                        return Err("Incompartible Y argument for the metrics math operations".into())
                    }
                };
                let x_val = x.clone();
                return Result::Ok(numeric_op_metric_number(op, x_val, y_val));
            }
            Val::Json(j_x) => {
                match y.data {
                    Val::Json(j_y) => {
                        return Result::Ok(Value::json(numeric_op_json_json(op, j_x, j_y)));
                    }
                    _ => return Err("Incompartible X and Y argument for the JSON math operations".into()),
                }
            }
            Val::F64(f_x) => {
                match y.data {
                    Val::F64(f_y) => {
                        match op {
                            Ops::Div => {
                                if f_y == 0.0 {
                                    return Err("Float-point division to 0.0".into());
                                }
                            }
                            _ => {},
                        }
                        return Result::Ok(Value::from(numeric_op_float_float(op, f_x, f_y)).unwrap());
                    }
                    Val::I64(i_y) => {
                        match op {
                            Ops::Div => {
                                if i_y == 0 {
                                    return Err("Integer division to 0.0".into());
                                }
                            }
                            _ => {},
                        }
                        return Result::Ok(Value::from(numeric_op_float_float(op, f_x, i_y as f64)).unwrap());
                    }
                    Val::String(s_y) => {
                        return Result::Ok(Value::from(string_op_string_int(op, s_y, f_x as i64)).unwrap());
                    }
                    _ => return Err("Incompartible X argument for the math operations".into()),
                }
            }
            Val::I64(i_x) => {
                match y.data {
                    Val::F64(f_y) => {
                        match op {
                            Ops::Div => {
                                if f_y == 0.0 {
                                    return Err("Float-point division to 0.0".into());
                                }
                            }
                            _ => {},
                        }
                        return Result::Ok(Value::from(numeric_op_float_float(op, i_x as f64, f_y)).unwrap());
                    }
                    Val::I64(i_y) => {
                        match op {
                            Ops::Div => {
                                if i_y == 0 {
                                    return Err("Integer division to 0.0".into());
                                }
                            }
                            _ => {},
                        }
                        return Result::Ok(Value::from(numeric_op_int_int(op, i_x, i_y)).unwrap());
                    }
                    Val::String(s_y) => {
                        return Result::Ok(Value::from(string_op_string_int(op, s_y, i_x)).unwrap());
                    }
                    _ => return Err("Incompartible Y argument for the math operations".into()),
                }
            }
            Val::Binary(b_x) => {
                match y.data {
                    Val::Binary(b_y) => {
                        match op {
                            Ops::Add => {
                                let mut res = Value::from_bin(b_x);
                                return Result::Ok(res.push(Value::from_bin(b_y)));
                            }
                            _ => {
                                return Err("Unsupported operation for binary".into());
                            }
                        }
                    }
                    Val::String(b_y) => {
                        match op {
                            Ops::Add => {
                                let mut res = Value::from_bin(b_x);
                                return Result::Ok(res.push(Value::from_string(b_y)));
                            }
                            _ => {
                                return Err("Unsupported operation for binary".into());
                            }
                        }
                    }
                    _ => {
                        return Err("Unsupported datatype Y for binary".into());
                    }
                }
            }
            Val::String(s_x) => {
                match y.data {
                    Val::String(s_y) => {
                        match x.dt {
                            TEXTBUFFER => {
                                return Result::Ok(Value::text_buffer(string_op_string_string(op, s_x, s_y)));
                            }
                            _ => {
                                return Result::Ok(Value::from(string_op_string_string(op, s_x, s_y)).unwrap());
                            }
                        }
                    }
                    Val::I64(i_y) => {
                        match x.dt {
                            TEXTBUFFER => {
                                return Result::Ok(Value::text_buffer(string_op_string_int(op, s_x, i_y)));
                            }
                            _ => {
                                return Result::Ok(Value::from(string_op_string_int(op, s_x, i_y)).unwrap());
                            }
                        }
                    }
                    Val::F64(f_y) => {
                        match x.dt {
                            TEXTBUFFER => {
                                return Result::Ok(Value::text_buffer(string_op_string_float(op, s_x, f_y)));
                            }
                            _ => {
                                return Result::Ok(Value::from(string_op_string_float(op, s_x, f_y)).unwrap());
                            }
                        }
                    }
                    _ => {
                        match y.conv(STRING) {
                            Ok(str_y) => {
                                match str_y.cast_string() {
                                    Ok(s_y) => {
                                        match x.dt {
                                            TEXTBUFFER => {
                                                return Result::Ok(Value::text_buffer(string_op_string_string(op, s_x, s_y)));
                                            }
                                            _ => {
                                                return Result::Ok(Value::from(string_op_string_string(op, s_x, s_y)).unwrap());
                                            }
                                        }
                                    }
                                    Err(err) => {
                                            return Err(format!("Incompartible Y argument for the string operations: {}", err).into());
                                    }
                                }
                            }
                            Err(err) => {
                                return Err(format!("Incompartible Y argument for the string operations: {}", err).into());
                            }
                        }
                    }
                }
            }
            _ => {
                match x.dt {
                    LIST => {
                        match y.dt {
                            LIST => {
                                match op {
                                    Ops::Add => {
                                        let mut res = Value::list();
                                        for v in x {
                                            res = res.push(v);
                                        }
                                        for v in y {
                                            res = res.push(v);
                                        }
                                        return Result::Ok(res);
                                    }
                                    _ => {
                                        return Err("Incompartible operation for the list".into());
                                    }
                                }
                            }
                            _ => {
                                match op {
                                    Ops::Add => {
                                        return Result::Ok(x.push(y));
                                    }
                                    _ => {
                                        return Err("Incompartible operation for the list".into());
                                    }
                                }
                            }
                        }
                    }
                    MATRIX => {
                        match y.dt {
                            MATRIX => {
                                return Result::Ok(Value::from_matrix(numeric_op_matrix(op, x.cast_matrix().unwrap(), y.cast_matrix().unwrap())));
                            }
                            LIST => {
                                match op {
                                    Ops::Add => {
                                        let mut res = Value::matrix();

                                        for v in x {
                                            res = res.push(v);
                                        }
                                        res = res.push(y);
                                        return Result::Ok(res);
                                    }
                                    _ => {
                                        return Err("Incompartible operation for the list".into());
                                    }
                                }
                            }
                            _ => {
                                return Err("Incompartible operation for the matrix".into());
                            }
                        }
                    }
                    CINTEGER => {
                        match y.dt {
                            CINTEGER => {
                                return Result::Ok(Value::from(numeric_op_cpx_int_cpx_int(op, x.cast_complex_int().unwrap(), y.cast_complex_int().unwrap())).unwrap());
                            }
                            _ => return Err("Incompartible Y argument for the math operations".into()),
                        }
                    }
                    CFLOAT => {
                        match y.dt {
                            CFLOAT => {
                                return Result::Ok(Value::from(numeric_op_cpx_float_cpx_float(op, x.cast_complex_float().unwrap(), y.cast_complex_float().unwrap())).unwrap());
                            }
                            _ => return Err("Incompartible Y argument for the math operations".into()),
                        }
                    }
                    _ => {
                        match y.dt {
                            LIST => {
                                match x.dt {
                                    LIST => {
                                        match op {
                                            Ops::Add => {
                                                let mut res = Value::list();
                                                for v in y {
                                                    res = res.push(v);
                                                }
                                                for v in x {
                                                    res = res.push(v);
                                                }
                                                return Result::Ok(res);
                                            }
                                            _ => {
                                                return Err("Incompartible operation for the list".into());
                                            }
                                        }
                                    }
                                    _ => {
                                        match op {
                                            Ops::Add => {
                                                return Result::Ok(y.push(x));
                                            }
                                            _ => {
                                                return Err("Incompartible operation for the list".into());
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                return Err(format!("Incompartible X argument for the math operations: {}", x.dt).into());
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Add for Value {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let q = (&self.get_q() + &other.get_q())/2.0;
        match Value::numeric_op(Ops::Add, self, other) {
            Ok(mut res) => {
                return res.set_q(q).clone();
            }
            Err(_) => Value::nodata(),
        }
    }
}

impl Sub for Value {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let q = (&self.get_q() + &other.get_q())/2.0;
        match Value::numeric_op(Ops::Sub, self, other) {
            Ok(mut res) => res.set_q(q).clone(),
            Err(_) => Value::nodata(),
        }
    }
}

impl Mul for Value {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let q = (&self.get_q() + &other.get_q())/2.0;
        match Value::numeric_op(Ops::Mul, self, other) {
            Ok(mut res) => res.set_q(q).clone(),
            Err(_) => Value::nodata(),
        }
    }
}

impl Div for Value {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let q = (&self.get_q() + &other.get_q())/2.0;
        match Value::numeric_op(Ops::Div, self, other) {
            Ok(mut res) => res.set_q(q).clone(),
            Err(_) => Value::nodata(),
        }
    }
}
