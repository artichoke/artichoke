use std::ffi::CStr;

use crate::extn::prelude::*;

const MONITOR_CSTR: &CStr = cstr::cstr!("Monitor");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = class::Spec::new("Monitor", MONITOR_CSTR, None, None)?;
    interp.def_class::<Monitor>(spec)?;
    interp.def_rb_source_file("monitor.rb", &include_bytes!("vendor/monitor.rb")[..])?;
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Monitor;

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;

    use crate::test::prelude::*;

    const SUBJECT: &str = "Monitor";
    const FUNCTIONAL_TEST: &[u8] = include_bytes!("monitor_test.rb");

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
