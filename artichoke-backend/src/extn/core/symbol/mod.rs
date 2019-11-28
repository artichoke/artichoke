use artichoke_core::eval::Eval;

use crate::class;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    if interp.0.borrow().class_spec::<Symbol>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("Symbol", None, None);
    interp.0.borrow_mut().def_class::<Symbol>(&spec);
    interp.eval(&include_bytes!("symbol.rb")[..])?;
    trace!("Patched Symbol onto interpreter");
    Ok(())
}

pub struct Symbol;
