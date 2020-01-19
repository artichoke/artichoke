use artichoke_core::eval::Eval;

use crate::class;
use crate::{Artichoke, BootError};

pub fn init(interp: &Artichoke) -> Result<(), BootError> {
    if interp.0.borrow().class_spec::<Method>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("Method", None, None)?;
    interp.0.borrow_mut().def_class::<Method>(spec);
    let _ = interp.eval(&include_bytes!("method.rb")[..])?;
    trace!("Patched Method onto interpreter");
    Ok(())
}

pub struct Method;
