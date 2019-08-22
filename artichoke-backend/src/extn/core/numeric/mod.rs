use crate::eval::Eval;
use crate::Artichoke;
use crate::ArtichokeError;

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp.borrow_mut().def_class::<Numeric>("Numeric", None, None);
    interp.eval(include_str!("numeric.rb"))?;
    Ok(())
}

pub struct Numeric;
