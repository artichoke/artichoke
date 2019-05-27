use crate::interpreter::Mrb;
use crate::MrbError;

pub mod error;
pub mod kernel;
pub mod module;
pub mod regexp;

pub fn patch(interp: &Mrb) -> Result<(), MrbError> {
    error::patch(interp)?;
    kernel::patch(interp)?;
    module::patch(interp)?;
    regexp::init(interp)?;
    Ok(())
}
