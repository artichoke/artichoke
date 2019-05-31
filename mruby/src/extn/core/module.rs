use crate::eval::MrbEval;
use crate::interpreter::Mrb;
use crate::MrbError;

pub fn patch(interp: &Mrb) -> Result<(), MrbError> {
    interp
        .borrow_mut()
        .def_class::<Module>("Module", None, None);
    interp.eval(include_str!("module.rb"))?;
    Ok(())
}

pub struct Module;
