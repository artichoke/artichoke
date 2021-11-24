pub mod mruby;
pub mod trampoline;

#[derive(Debug, Clone, Copy)]
pub struct String;

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    const SUBJECT: &str = "String";
    const FUNCTIONAL_TEST: &[u8] = include_bytes!("string_test.rb");

    #[test]
    #[cfg(feature = "core-regexp")]
    fn functional() {
        let mut interp = interpreter().unwrap();
        let result = interp.eval(FUNCTIONAL_TEST);
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
        let result = interp.eval(b"spec");
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
    }
}
