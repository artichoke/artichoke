use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = module::Spec::new(interp, "Forwardable", None)?;
    interp.def_module::<Forwardable>(spec)?;
    interp.def_rb_source_file(
        "forwardable.rb",
        &include_bytes!("vendor/forwardable.rb")[..],
    )?;
    interp.def_rb_source_file(
        "forwardable/impl.rb",
        &include_bytes!("vendor/forwardable/impl.rb")[..],
    )?;
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Forwardable;

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;
    use bstr::ByteSlice;

    const SUBJECT: &str = "Forwardable";
    const FUNCTIONAL_TEST: &[u8] = include_bytes!("forwardable_test.rb");

    #[test]
    // TODO(GH-528): fix failing tests on Windows.
    #[cfg_attr(target_os = "windows", should_panic)]
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
