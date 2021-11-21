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
    use bstr::ByteSlice;

    use crate::test::prelude::*;

    const SUBJECT: &str = "Abbrev";
    const FUNCTIONAL_TEST: &[u8] = include_bytes!("abbrev_test.rb");

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
