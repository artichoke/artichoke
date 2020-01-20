use artichoke_core::eval::Eval;

use crate::extn::prelude::*;

pub fn init(interp: &Artichoke) -> Result<(), BootError> {
    if interp.0.borrow().class_spec::<Enumerator>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("Enumerator", None, None)?;
    interp.0.borrow_mut().def_class::<Enumerator>(spec);
    let _ = interp.eval(&include_bytes!("enumerator.rb")[..])?;
    let _ = interp.eval(&include_bytes!("lazy.rb")[..])?;
    trace!("Patched Enumerator onto interpreter");
    trace!("Patched Enumerator::Lazy onto interpreter");
    Ok(())
}

pub struct Enumerator;
