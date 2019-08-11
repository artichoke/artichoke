use log::trace;

use crate::def::Define;
use crate::eval::Eval;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    if interp.borrow().module_spec::<Comparable>().is_some() {
        return Ok(());
    }

    let comparable = interp
        .borrow_mut()
        .def_module::<Comparable>("Comparable", None);
    comparable.borrow().define(interp)?;
    interp.eval(include_str!("comparable.rb"))?;

    trace!("Patched Comparable onto interpreter");

    Ok(())
}

pub struct Comparable;
