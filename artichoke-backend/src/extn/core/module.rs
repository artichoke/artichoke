use crate::eval::Eval;
use crate::Artichoke;
use crate::ArtichokeError;

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp
        .borrow_mut()
        .def_class::<Module>("Module", None, None);
    interp.eval(include_str!("module.rb"))?;
    Ok(())
}

pub struct Module;
