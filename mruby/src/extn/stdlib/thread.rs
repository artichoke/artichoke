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
