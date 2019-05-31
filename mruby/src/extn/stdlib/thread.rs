use crate::eval::MrbEval;
use crate::interpreter::Mrb;
use crate::load::MrbLoadSources;
use crate::MrbError;

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    interp
        .borrow_mut()
        .def_class::<Thread>("Thread", None, None);
    interp
        .borrow_mut()
        .def_class::<Mutex>("Mutex", None, None);
    interp.def_rb_source_file("thread.rb", include_str!("thread.rb"))?;
    // Thread is loaded by default, so require it on interpreter initialization
    // https://www.rubydoc.info/gems/rubocop/RuboCop/Cop/Lint/UnneededRequireStatement
    interp.eval("require 'thread'")?;
    Ok(())
}

pub struct Thread;
pub struct Mutex;

#[cfg(test)]
mod tests {
    use crate::convert::TryFromMrb;
    use crate::eval::MrbEval;
    use crate::interpreter::Interpreter;

    #[test]
    fn thread_required_by_default() {
        let interp = Interpreter::create().expect("mrb init");
        let result = interp.eval("Object.const_defined?(:Thread)").expect("thread");
        assert!(unsafe { bool::try_from_mrb(&interp, result) }.expect("convert"));
    }
}
