use std::ffi::CStr;

use crate::extn::prelude::*;

const MONITOR_CSTR: &CStr = cstr::cstr!("Monitor");
static MONITOR_RUBY_SOURCE: &[u8] = include_bytes!("vendor/monitor.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = class::Spec::new("Monitor", MONITOR_CSTR, None, None)?;
    interp.def_class::<Monitor>(spec)?;
    interp.def_rb_source_file("monitor.rb", MONITOR_RUBY_SOURCE)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Monitor;

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    const SUBJECT: &str = "Monitor";
    const FUNCTIONAL_TEST: &[u8] = include_bytes!("monitor_test.rb");

    #[test]
    fn functional() {
        let mut interp = interpreter().unwrap();
        let result = interp.eval(FUNCTIONAL_TEST);
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
        let result = interp.eval(b"spec");
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
    }
}
