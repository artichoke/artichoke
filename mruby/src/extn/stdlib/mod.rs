use crate::eval::MrbEval;
use crate::interpreter::Mrb;
use crate::MrbError;

pub mod monitor;
pub mod thread;

pub fn patch(interp: &Mrb) -> Result<(), MrbError> {
    monitor::init(interp)?;
    thread::init(interp)?;
    // https://www.rubydoc.info/gems/rubocop/RuboCop/Cop/Lint/UnneededRequireStatement
    interp.eval("require 'thread'")?;
    Ok(())
}
