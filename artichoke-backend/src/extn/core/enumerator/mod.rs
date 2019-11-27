use artichoke_core::eval::Eval;

use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp
        .0
        .borrow_mut()
        .def_class::<Enumerator>("Enumerator", None, None);
    interp.eval(&include_bytes!("enumerator.rb")[..])?;
    interp.eval(&include_bytes!("lazy.rb")[..])?;
    Ok(())
}

pub struct Enumerator;
