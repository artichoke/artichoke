use crate::eval::MrbEval;
use crate::Mrb;
use crate::MrbError;

pub fn patch(interp: &Mrb) -> Result<(), MrbError> {
    interp.borrow_mut().def_class::<Hash>("Hash", None, None);
    interp.eval(include_str!("hash.rb"))?;
    Ok(())
}

pub struct Hash;
