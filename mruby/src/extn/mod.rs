use crate::interpreter::Mrb;
use crate::MrbError;

pub mod core;

pub fn patch(interp: &Mrb) -> Result<(), MrbError> {
    core::patch(interp)?;
    Ok(())
}
