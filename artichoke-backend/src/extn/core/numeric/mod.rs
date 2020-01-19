use artichoke_core::eval::Eval;

use crate::class;
use crate::{Artichoke, BootError};

pub fn init(interp: &Artichoke) -> Result<(), BootError> {
    if interp.0.borrow().class_spec::<Numeric>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("Numeric", None, None)?;
    interp.0.borrow_mut().def_class::<Numeric>(spec);
    let _ = interp.eval(&include_bytes!("numeric.rb")[..])?;
    trace!("Patched Numeric onto interpreter");
    Ok(())
}

pub struct Numeric;
