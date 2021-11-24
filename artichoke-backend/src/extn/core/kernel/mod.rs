pub mod integer;
pub mod mruby;
pub mod require;
pub mod trampoline;

#[derive(Debug, Clone, Copy)]
pub struct Kernel;

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    const SUBJECT: &str = "Kernel";
    const FUNCTIONAL_TEST: &[u8] = include_bytes!("kernel_test.rb");

    #[test]
    fn functional() {
        let mut interp = interpreter().unwrap();
        let result = interp.eval(FUNCTIONAL_TEST);
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
        let result = interp.eval(b"spec");
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
    }
}
