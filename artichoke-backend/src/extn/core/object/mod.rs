use artichoke_core::eval::Eval;

use crate::class;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    if interp.0.borrow().class_spec::<Object>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("Object", None, None)?;
    interp.0.borrow_mut().def_class::<Object>(spec);
    let _ = interp.eval(&include_bytes!("object.rb")[..])?;
    trace!("Patched Object onto interpreter");
    Ok(())
}

pub struct Object;
