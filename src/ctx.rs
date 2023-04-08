use crate::value::{Value};

pub type CtxAppFn  = fn(&dyn Context,&str,Value) -> Option<Value>;

#[derive(Clone)]
pub struct CtxApplicative {
    pub name:   String,
    pub f:      CtxAppFn,
}

impl CtxApplicative {
    pub fn new<N: AsRef<str> + std::fmt::Display>(name: N, f: CtxAppFn) -> Self {
        Self {
            name: name.as_ref().to_string(),
            f:    f,
        }
    }
}


pub trait Context {
    fn new() -> Self where Self: Sized;
    fn resolve(&self, name: &str) -> Option<CtxApplicative>;
    fn get_association(&self, name: &str) -> Option<Value>;
    fn register(&mut self, name: &str, f: CtxApplicative) -> bool;
    fn register_association(&mut self, name: &str, v: Value) -> bool;
    fn eval(&mut self, v: Value) -> Option<Value>;
}

pub struct NullContext {}

impl Context for NullContext {
    fn new() -> NullContext {
        Self {

        }
    }
    fn resolve(&self, _name: &str) -> Option<CtxApplicative> {
        fn none_fn(_ctx: &dyn Context, _name: &str, _value: Value) -> Option<Value> {
            None
        }
        Some(CtxApplicative::new("none_fn", none_fn))
    }
    fn get_association(&self, _name: &str) -> Option<Value> {
        None
    }
    fn register(&mut self, _name: &str, _f: CtxApplicative) -> bool {
        true
    }
    fn register_association(&mut self, _name: &str, _v: Value) -> bool {
        true
    }
    fn eval(&mut self, _v: Value) -> Option<Value> {
        None
    }
}


pub fn context(mut ctx: impl Context, value: Value) -> Option<Value> {
    ctx.eval(value)
}
