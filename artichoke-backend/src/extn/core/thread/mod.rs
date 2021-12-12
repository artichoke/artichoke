use std::ffi::CStr;

use crate::extn::prelude::*;

const THREAD_CSTR: &CStr = cstr::cstr!("Thread");
const MUTEX_CSTR: &CStr = cstr::cstr!("Mutex");
static THREAD_RUBY_SOURCE: &[u8] = include_bytes!("thread.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Thread>() {
        return Ok(());
    }
    if interp.is_class_defined::<Mutex>() {
        return Ok(());
    }

    let spec = class::Spec::new("Thread", THREAD_CSTR, None, None)?;
    interp.def_class::<Thread>(spec)?;
    let spec = class::Spec::new("Mutex", MUTEX_CSTR, None, None)?;
    interp.def_class::<Mutex>(spec)?;
    // TODO: Don't add a source file and don't add an explicit require below.
    // Instead, have thread be a default loaded feature in `mezzaluna-feature-loader`.
    interp.def_rb_source_file("thread.rb", THREAD_RUBY_SOURCE)?;
    // Thread is loaded by default, so eval it on interpreter initialization
    // https://www.rubydoc.info/gems/rubocop/RuboCop/Cop/Lint/UnneededRequireStatement
    interp.eval(&b"require 'thread'"[..])?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Thread;

#[derive(Debug, Clone, Copy)]
pub struct Mutex;

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    const SUBJECT: &str = "Thread";
    const FUNCTIONAL_TEST: &[u8] = include_bytes!("thread_test.rb");

    #[test]
    fn functional() {
        let mut interp = interpreter().unwrap();
        let result = interp.eval(FUNCTIONAL_TEST);
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
        let result = interp.eval(b"spec");
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
    }
}
