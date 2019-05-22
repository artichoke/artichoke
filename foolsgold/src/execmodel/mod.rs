use mruby::eval::MrbEval;
use mruby::interpreter::{Interpreter, Mrb};
use mruby::MrbError;

use crate::foolsgold;

pub mod prefork;
pub mod shared_nothing;

fn interpreter() -> Result<Mrb, MrbError> {
    let mut interp = Interpreter::create()?;
    nemesis::init(&mut interp)?;
    foolsgold::init(&mut interp)?;
    // preload foolsgold sources
    interp.eval("require 'foolsgold'")?;
    interp.eval("require 'foolsgold/adapter/memory'")?;
    Ok(interp)
}
