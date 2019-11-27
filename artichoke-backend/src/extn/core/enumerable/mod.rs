use artichoke_core::eval::Eval;

use crate::def::Define;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    if interp.0.borrow().module_spec::<Enumerable>().is_some() {
        return Ok(());
    }

    let enumerable = interp
        .0
        .borrow_mut()
        .def_module::<Enumerable>("Enumerable", None);
    enumerable.borrow().define(interp)?;
    interp.eval(&include_bytes!("enumerable.rb")[..])?;

    Ok(())
}

pub struct Enumerable;
