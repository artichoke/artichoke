use crate::interpreter::Mrb;
use crate::MrbError;

#[allow(clippy::module_name_repetitions)]
pub trait MrbFile {
    fn require(interp: Mrb) -> Result<(), MrbError>;
}
