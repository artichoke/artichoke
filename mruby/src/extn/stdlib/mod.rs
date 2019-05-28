use crate::interpreter::Mrb;
use crate::MrbError;

pub mod monitor;
pub mod thread;

pub fn patch(interp: &Mrb) -> Result<(), MrbError> {
    monitor::init(interp)?;
    thread::init(interp)?;
    Ok(())
}
