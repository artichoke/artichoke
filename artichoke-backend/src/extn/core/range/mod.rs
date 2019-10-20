use crate::eval::Eval;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp
        .0
        .borrow_mut()
        .def_class::<Range>("Range", None, None);
    interp.eval(include_str!("range.rb"))?;
    Ok(())
}

pub struct Range;
