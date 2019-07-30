use crate::eval::Eval;
use crate::Artichoke;
use crate::ArtichokeError;

pub mod array;
pub mod env;
pub mod error;
pub mod hash;
pub mod kernel;
#[cfg(not(target_arch = "wasm32"))]
pub mod matchdata;
pub mod module;
#[cfg(not(target_arch = "wasm32"))]
pub mod regexp;
pub mod string;
pub mod thread;

pub fn patch(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp.eval(include_str!("object.rb"))?;
    array::patch(interp)?;
    env::patch(interp)?;
    error::patch(interp)?;
    hash::patch(interp)?;
    kernel::patch(interp)?;
    #[cfg(not(target_arch = "wasm32"))]
    matchdata::init(interp)?;
    module::patch(interp)?;
    #[cfg(not(target_arch = "wasm32"))]
    regexp::init(interp)?;
    string::patch(interp)?;
    thread::init(interp)?;
    Ok(())
}
