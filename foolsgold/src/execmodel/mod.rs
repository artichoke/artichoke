use mruby::eval::MrbEval;
use mruby::interpreter::{Interpreter, Mrb};
use mruby::MrbError;

use crate::foolsgold;

pub mod prefork;
pub mod shared_nothing;

fn interpreter() -> Result<Mrb, MrbError> {
    let interp = Interpreter::create()?;
    nemesis::init(&interp)?;
    foolsgold::init(&interp)?;
    // preload foolsgold sources
    interp.eval("require 'foolsgold'")?;
    Ok(interp)
}
