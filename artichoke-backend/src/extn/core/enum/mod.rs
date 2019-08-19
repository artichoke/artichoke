use crate::eval::Eval;
use crate::Artichoke;
use crate::ArtichokeError;

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp.borrow_mut().def_module::<Enumerable>("Enumerable", None, None);
    interp.eval(include_str!("enum.rb"))?;
    Ok(())
}

pub struct Enumerable;
