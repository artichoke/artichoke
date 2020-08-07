use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = class::Spec::new("StringScanner", None, None)?;
    interp.def_class::<StringScanner>(spec)?;
    interp.def_rb_source_file("strscan.rb", &include_bytes!("strscan.rb")[..])?;
    Ok(())
}

#[derive(Debug)]
pub struct StringScanner;

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;
    use bstr::ByteSlice;

    const SUBJECT: &str = "StringScanner";
    const FUNCTIONAL_TEST: &[u8] = include_bytes!("strscan_test.rb");

    #[test]
    fn functional() {
        let mut interp = crate::interpreter().unwrap();
        let _ = interp.eval(FUNCTIONAL_TEST).unwrap();
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
