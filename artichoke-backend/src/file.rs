use crate::{Artichoke, ArtichokeError};

pub trait File {
    fn require(interp: Artichoke) -> Result<(), ArtichokeError>;
}
