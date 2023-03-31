use crate::value::{Applicative, Value};

pub type CtxAppFn  = fn(dyn Context,Value) -> Value;

pub trait Context {
    fn new() -> Self;
    fn resolve<N: AsRef<str>>(&self, name: N) -> Option<Applicative>;
    fn get_association<N: AsRef<str>>(&self, name: N) -> Option<Value>;
}

pub struct NullContext {}

impl Context for NullContext {
    fn new() -> NullContext {
        Self {

        }
    }
    fn resolve<N: AsRef<str>>(&self, _name: N) -> Option<Applicative> {
        None
    }
    fn get_association<N: AsRef<str>>(&self, _name: N) -> Option<Value> {
        None
    }
}
