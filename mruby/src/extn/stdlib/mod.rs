use crate::interpreter::Mrb;
use crate::MrbError;

pub mod delegate;
pub mod forwardable;
pub mod monitor;
pub mod set;

pub fn patch(interp: &Mrb) -> Result<(), MrbError> {
    delegate::init(interp)?;
    forwardable::init(interp)?;
    monitor::init(interp)?;
    set::init(interp)?;
    Ok(())
}
