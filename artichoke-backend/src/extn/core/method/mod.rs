use crate::eval::Eval;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp
        .0
        .borrow_mut()
        .def_class::<Method>("Method", None, None);
    interp.eval(&include_bytes!("method.rb")[..])?;
    Ok(())
}

pub struct Method;
