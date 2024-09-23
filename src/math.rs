use std::ops::{Add, Sub, Mul, Div};
use num::complex::Complex;
use crate::value::Value;
use crate::types::*;

pub enum Ops {
    Add,
    Sub,
    Mul,
    Div,
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
        Ops::Add => format!("{}{}", x, y),
        _ => x,
    }
}

impl Value {
    pub fn numeric_op(op: Ops, mut x: Value, mut y: Value) -> Result<Value, Box<dyn std::error::Error>> {
        match x.data {
            Val::F64(f_x) => {
                match y.data {
                    Val::F64(f_y) => {
                        return Result::Ok(Value::from(numeric_op_float_float(op, f_x, f_y)).unwrap());
                    }
                    Val::I64(i_y) => {
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
                        return Result::Ok(Value::from(numeric_op_float_float(op, i_x as f64, f_y)).unwrap());
                    }
                    Val::I64(i_y) => {
                        return Result::Ok(Value::from(numeric_op_int_int(op, i_x, i_y)).unwrap());
                    }
                    Val::String(s_y) => {
                        return Result::Ok(Value::from(string_op_string_int(op, s_y, i_x)).unwrap());
                    }
                    _ => return Err("Incompartible Y argument for the math operations".into()),
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
