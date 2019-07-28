use crate::{ArtichokeError, Mrb};

#[allow(clippy::module_name_repetitions)]
pub trait MrbFile {
    fn require(interp: Mrb) -> Result<(), ArtichokeError>;
}
