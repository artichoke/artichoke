use crate::eval::Eval;
use crate::Artichoke;
use crate::ArtichokeError;

pub mod array;
pub mod comparable;
pub mod env;
pub mod error;
pub mod hash;
pub mod kernel;
pub mod matchdata;
pub mod module;
pub mod regexp;
pub mod string;
pub mod thread;

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp.eval(include_str!("object.rb"))?;
    array::init(interp)?;
    comparable::init(interp)?;
    env::init(interp)?;
    error::init(interp)?;
    hash::init(interp)?;
    kernel::init(interp)?;
    matchdata::init(interp)?;
    module::init(interp)?;
    regexp::init(interp)?;
    string::init(interp)?;
    thread::init(interp)?;
    Ok(())
}
