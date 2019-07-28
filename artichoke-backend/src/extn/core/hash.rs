use crate::eval::Eval;
use crate::ArtichokeError;
use crate::Mrb;

pub fn patch(interp: &Mrb) -> Result<(), ArtichokeError> {
    interp.borrow_mut().def_class::<Hash>("Hash", None, None);
    interp.eval(include_str!("hash.rb"))?;
    Ok(())
}

pub struct Hash;
