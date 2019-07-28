use crate::{ArtichokeError, Mrb};

pub trait File {
    fn require(interp: Mrb) -> Result<(), ArtichokeError>;
}
