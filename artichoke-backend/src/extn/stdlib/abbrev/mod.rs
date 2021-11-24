use std::ffi::CStr;

use crate::extn::prelude::*;

const ABBREV_CSTR: &CStr = cstr::cstr!("Abbrev");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = module::Spec::new(interp, "Abbrev", ABBREV_CSTR, None)?;
    interp.def_module::<Abbrev>(spec)?;
    interp.def_rb_source_file("abbrev.rb", &include_bytes!("vendor/abbrev.rb")[..])?;
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Abbrev;

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    const SUBJECT: &str = "Abbrev";
    const FUNCTIONAL_TEST: &[u8] = include_bytes!("abbrev_test.rb");

    #[test]
    fn functional() {
        let mut interp = interpreter().unwrap();
        let result = interp.eval(FUNCTIONAL_TEST);
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
        let result = interp.eval(b"spec");
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
    }
}
