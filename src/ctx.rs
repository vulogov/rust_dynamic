use crate::value::{Value};

pub type CtxAppFn  = fn(&dyn Context,Value) -> Option<Value>;

pub struct CtxApplicative {
    pub f:      CtxAppFn,
}

pub trait Context {
    fn new() -> Self where Self: Sized;
    fn resolve(&self, name: &str) -> Option<CtxAppFn>;
    fn get_association(&self, name: &str) -> Option<Value>;
}

pub struct NullContext {}

impl Context for NullContext {
    fn new() -> NullContext {
        Self {

        }
    }
    fn resolve(&self, _name: &str) -> Option<CtxAppFn> {
        fn none_fn(_ctx: &dyn Context, _value: Value) -> Option<Value> {
            None
        }
        Some(none_fn)
    }
    fn get_association(&self, _name: &str) -> Option<Value> {
        None
    }
}


pub fn context(_ctx: impl Context, _name: &str, _value: Value) -> Option<Value> {
    fn none_fun(_ctx: impl Context, _value: Value) -> Option<Value>{
        Some(Value::none())
    }
    none_fun(_ctx, _value)
}
