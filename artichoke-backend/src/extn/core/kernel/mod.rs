pub mod integer;
pub mod mruby;
pub mod require;
pub mod trampoline;

#[derive(Debug, Clone, Copy)]
pub struct Kernel;

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;

    use crate::test::prelude::*;

    const SUBJECT: &str = "Kernel";
    const FUNCTIONAL_TEST: &[u8] = include_bytes!("kernel_test.rb");

    #[test]
    fn functional() {
        let mut interp = interpreter().unwrap();
        interp.eval(FUNCTIONAL_TEST).unwrap();
        let result = interp.eval(b"spec");
        if let Err(exc) = result {
            let backtrace = exc.vm_backtrace(&mut interp);
            let backtrace = bstr::join("\n", backtrace.unwrap_or_default());
            panic!(
                "{} tests failed with message: {:?} and backtrace:\n{:?}",
                SUBJECT,
                exc.message().as_bstr(),
                backtrace.as_bstr()
            );
        }
    }
}
