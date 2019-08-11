use crate::eval::Eval;
use crate::Artichoke;
use crate::ArtichokeError;

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp.borrow_mut().def_class::<Hash>("Hash", None, None);
    interp.eval(include_str!("hash.rb"))?;
    Ok(())
}

pub struct Hash;
