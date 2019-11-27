use artichoke_core::eval::Eval;

use crate::def::Define;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    if interp.0.borrow().module_spec::<Comparable>().is_some() {
        return Ok(());
    }

    let comparable = interp
        .0
        .borrow_mut()
        .def_module::<Comparable>("Comparable", None);
    comparable.borrow().define(interp)?;
    interp.eval(&include_bytes!("comparable.rb")[..])?;

    trace!("Patched Comparable onto interpreter");

    Ok(())
}

pub struct Comparable;
