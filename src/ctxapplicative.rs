use crate::value::{Value};
use crate::ctx::{Context, CtxApplicative, CtxAppFn};

impl CtxApplicative {
    pub fn bind(&mut self, f: CtxAppFn) -> Self {
        self.f = f;
        return self.clone();
    }
    pub fn apply<N: AsRef<str> + std::fmt::Display>(&self, ctx: &dyn Context, name: N, value: Value) -> Option<Value> {
        let res = value.dup().unwrap().regen_id();
        return (self.f)(ctx, name.as_ref(), res);
    }
}
