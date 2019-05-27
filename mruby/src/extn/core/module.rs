use crate::eval::MrbEval;
use crate::interpreter::Mrb;
use crate::MrbError;

const PATCH: &str = include_str!("module.rb");

pub fn patch(interp: &Mrb) -> Result<(), MrbError> {
    interp
        .borrow_mut()
        .def_class::<Module>("Module", None, None);
    interp.eval(PATCH)?;
    Ok(())
}

pub struct Module;
