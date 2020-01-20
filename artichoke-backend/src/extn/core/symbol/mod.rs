use crate::extn::prelude::*;

pub fn init(interp: &Artichoke) -> Result<(), BootError> {
    if interp.0.borrow().class_spec::<Symbol>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("Symbol", None, None)?;
    interp.0.borrow_mut().def_class::<Symbol>(spec);
    let _ = interp.eval(&include_bytes!("symbol.rb")[..])?;
    trace!("Patched Symbol onto interpreter");
    Ok(())
}

pub struct Symbol;
