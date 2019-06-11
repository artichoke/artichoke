use crate::def::Define;
use crate::eval::MrbEval;
use crate::interpreter::Mrb;
use crate::MrbError;
use log::trace;

pub fn patch(interp: &Mrb) -> Result<(), MrbError> {
    let string = interp
        .borrow_mut()
        .def_class::<Symbol>("Symbol", None, None);
    interp.eval(include_str!("symbol.rb"))?;
    string.borrow().define(interp).map_err(|_| MrbError::New)?;
    trace!("Patched Symbol onto interpreter");
    Ok(())
}

pub struct Symbol;
