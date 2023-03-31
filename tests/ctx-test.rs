#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_dynamic::ctx::{Context, context};
    use rust_dynamic::ctx::{NullContext};


    #[test]
    fn test_null_ctx() {
        let ctx = NullContext::new();
        assert!(ctx.get_association("hello").is_none());
    }
    #[test]
    fn test_null_ctx_resolve() {
        let ctx = NullContext::new();
        let fun = ctx.resolve("test").expect("function expected");
        assert!(fun(&ctx, Value::none()).is_none());
    }
}
