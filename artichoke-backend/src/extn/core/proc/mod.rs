use crate::eval::Eval;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp.0.borrow_mut().def_class::<Proc>("Proc", None, None);
    interp.eval(&include_bytes!("proc.rb")[..])?;
    Ok(())
}

pub struct Proc;
