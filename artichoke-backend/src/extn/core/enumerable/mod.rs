use crate::def::Define;
use crate::eval::Eval;
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
    interp.eval(include_str!("enumerable.rb"))?;

    Ok(())
}

pub struct Enumerable;
