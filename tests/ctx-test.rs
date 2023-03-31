#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::ctx::Context;
    use rust_dynamic::ctx::{NullContext};


    #[test]
    fn test_null_ctx() {
        let ctx = NullContext::new();
        assert!(ctx.resolve("hello").is_none());
    }
}
