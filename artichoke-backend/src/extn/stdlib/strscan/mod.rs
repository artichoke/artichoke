use std::ffi::CStr;

use crate::extn::prelude::*;

const STRING_SCANNER_CSTR: &CStr = cstr::cstr!("StringScanner");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = class::Spec::new("StringScanner", STRING_SCANNER_CSTR, None, None)?;
    interp.def_class::<StringScanner>(spec)?;
    interp.def_rb_source_file("strscan.rb", &include_bytes!("strscan.rb")[..])?;
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct StringScanner;

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    const SUBJECT: &str = "StringScanner";
    const FUNCTIONAL_TEST: &[u8] = include_bytes!("strscan_test.rb");

    #[test]
    fn functional() {
        let mut interp = interpreter().unwrap();
        let result = interp.eval(FUNCTIONAL_TEST);
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
        let result = interp.eval(b"spec");
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
    }
}
