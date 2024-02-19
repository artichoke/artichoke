mod integer;
pub(in crate::extn) mod mruby;
pub mod require;
pub(super) mod trampoline;

#[allow(unused_imports)]
pub use trampoline::integer;

#[derive(Debug, Clone, Copy)]
pub struct Kernel;

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    const SUBJECT: &str = "Kernel";
    const FUNCTIONAL_TEST: &[u8] = include_bytes!("kernel_test.rb");

    #[test]
    fn functional() {
        let mut interp = interpreter();
        let result = interp.eval(FUNCTIONAL_TEST);
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
        let result = interp.eval(b"spec");
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
    }
}
