use crate::eval::MrbEval;
use crate::ArtichokeError;
use crate::Mrb;

pub fn patch(interp: &Mrb) -> Result<(), ArtichokeError> {
    interp.borrow_mut().def_class::<Array>("Array", None, None);
    interp.eval(include_str!("array.rb"))?;
    Ok(())
}

pub struct Array;
