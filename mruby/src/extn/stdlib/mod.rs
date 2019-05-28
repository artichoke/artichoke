use crate::interpreter::Mrb;
use crate::MrbError;

pub mod monitor;

pub fn patch(interp: &Mrb) -> Result<(), MrbError> {
    monitor::init(interp)?;
    Ok(())
}
