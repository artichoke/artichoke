use crate::eval::MrbEval;
use crate::interpreter::Mrb;
use crate::MrbError;

pub fn patch(interp: &Mrb) -> Result<(), MrbError> {
    interp.borrow_mut().def_class::<Array>("Array", None, None);
    interp.eval(include_str!("array.rb"))?;
    Ok(())
}

pub struct Array;
