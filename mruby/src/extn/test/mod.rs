use crate::interpreter::Mrb;
use crate::MrbError;

pub mod mspec;

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    mspec::init(interp)?;
    Ok(())
}
