use crate::eval::Eval;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp
        .0
        .borrow_mut()
        .def_class::<Symbol>("Symbol", None, None);
    interp.eval(include_str!("symbol.rb"))?;
    Ok(())
}

pub struct Symbol;
