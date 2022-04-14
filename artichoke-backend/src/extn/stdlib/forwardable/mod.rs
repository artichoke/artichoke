use std::ffi::CStr;

use crate::extn::prelude::*;

const FORWARDABLE_CSTR: &CStr = qed::const_cstr_from_str!("Forwardable\0");
static FORWARDABLE_RUBY_SOURCE: &[u8] = include_bytes!("vendor/forwardable.rb");
static FORWARDABLE_IMPL_RUBY_SOURCE: &[u8] = include_bytes!("vendor/forwardable/impl.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = module::Spec::new(interp, "Forwardable", FORWARDABLE_CSTR, None)?;
    interp.def_module::<Forwardable>(spec)?;
    interp.def_rb_source_file("forwardable.rb", FORWARDABLE_RUBY_SOURCE)?;
    interp.def_rb_source_file("forwardable/impl.rb", FORWARDABLE_IMPL_RUBY_SOURCE)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Forwardable;

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    const SUBJECT: &str = "Forwardable";
    const FUNCTIONAL_TEST: &[u8] = include_bytes!("forwardable_test.rb");

    #[test]
    fn functional() {
        let mut interp = interpreter();
        let result = interp.eval(FUNCTIONAL_TEST);
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
        let result = interp.eval(b"spec");
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
    }
}
