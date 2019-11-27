use artichoke_core::eval::Eval;

use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp
        .0
        .borrow_mut()
        .def_class::<Module>("Module", None, None);
    interp.eval(&include_bytes!("module.rb")[..])?;
    Ok(())
}

pub struct Module;
