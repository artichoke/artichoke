use crate::eval::Eval;
use crate::ArtichokeError;
use crate::Mrb;

pub fn patch(interp: &Mrb) -> Result<(), ArtichokeError> {
    interp
        .borrow_mut()
        .def_class::<Module>("Module", None, None);
    interp.eval(include_str!("module.rb"))?;
    Ok(())
}

pub struct Module;
