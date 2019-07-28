use crate::eval::MrbEval;
use crate::ArtichokeError;
use crate::Mrb;

pub mod array;
pub mod env;
pub mod error;
pub mod hash;
pub mod kernel;
pub mod matchdata;
pub mod module;
pub mod regexp;
pub mod string;
pub mod thread;

pub fn patch(interp: &Mrb) -> Result<(), ArtichokeError> {
    interp.eval(include_str!("object.rb"))?;
    array::patch(interp)?;
    env::patch(interp)?;
    error::patch(interp)?;
    hash::patch(interp)?;
    kernel::patch(interp)?;
    matchdata::init(interp)?;
    module::patch(interp)?;
    regexp::init(interp)?;
    string::patch(interp)?;
    thread::init(interp)?;
    Ok(())
}
