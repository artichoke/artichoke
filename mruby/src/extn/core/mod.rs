use crate::interpreter::Mrb;
use crate::MrbError;

pub mod error;
pub mod kernel;

pub fn patch(interp: &Mrb) -> Result<(), MrbError> {
    error::patch(interp)?;
    kernel::patch(interp)?;
    Ok(())
}
