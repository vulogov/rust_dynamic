use std::ops::{Add, Sub, Mul, Div};
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

impl Value {
    pub fn numeric_op(op: Ops, x: Value, y: Value) -> Result<Value, Box<dyn std::error::Error>> {
        match x.data {
            Val::F64(f_x) => {
                match y.data {
                    Val::F64(f_y) => {
                        return Result::Ok(Value::from(numeric_op_float_float(op, f_x, f_y)).unwrap());
                    }
                    Val::I64(i_y) => {
                        return Result::Ok(Value::from(numeric_op_float_float(op, f_x, i_y as f64)).unwrap());
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
                    _ => return Err("Incompartible X argument for the math operations".into()),
                }
            }
            _ => return Err("Incompartible X argument for the math operations".into()),
        }
    }
}

impl Add for Value {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match Value::numeric_op(Ops::Add, self, other) {
            Ok(res) => res,
            Err(_) => Value::nodata(),
        }
    }
}

impl Sub for Value {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        match Value::numeric_op(Ops::Sub, self, other) {
            Ok(res) => res,
            Err(_) => Value::nodata(),
        }
    }
}

impl Mul for Value {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        match Value::numeric_op(Ops::Mul, self, other) {
            Ok(res) => res,
            Err(_) => Value::nodata(),
        }
    }
}

impl Div for Value {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        match Value::numeric_op(Ops::Div, self, other) {
            Ok(res) => res,
            Err(_) => Value::nodata(),
        }
    }
}
