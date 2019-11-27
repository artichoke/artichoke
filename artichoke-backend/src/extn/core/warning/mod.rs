use artichoke_core::eval::Eval;

use crate::def::Define;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    let warning = interp.0.borrow_mut().def_module::<Warning>("Warning", None);
    warning.borrow().define(interp)?;
    interp.eval(&include_bytes!("warning.rb")[..])?;

    trace!("Patched Warning onto interpreter");
    Ok(())
}

pub struct Warning;
