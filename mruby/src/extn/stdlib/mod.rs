use crate::interpreter::Mrb;
use crate::MrbError;

pub mod forwardable;
pub mod monitor;

pub fn patch(interp: &Mrb) -> Result<(), MrbError> {
    forwardable::init(interp)?;
    monitor::init(interp)?;
    Ok(())
}
