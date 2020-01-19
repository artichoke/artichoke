use artichoke_core::eval::Eval;

use crate::class;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    if interp.0.borrow().class_spec::<Module>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("Module", None, None)?;
    interp.0.borrow_mut().def_class::<Module>(spec);
    let _ = interp.eval(&include_bytes!("module.rb")[..])?;
    trace!("Patched Module onto interpreter");
    Ok(())
}

pub struct Module;
