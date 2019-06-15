use crate::interpreter::Mrb;
use crate::MrbError;

pub mod core;
pub mod stdlib;
#[cfg(test)]
pub mod test;

pub fn patch(interp: &Mrb) -> Result<(), MrbError> {
    core::patch(interp)?;
    stdlib::patch(interp)?;
    #[cfg(test)]
    test::init(interp)?;
    Ok(())
}
