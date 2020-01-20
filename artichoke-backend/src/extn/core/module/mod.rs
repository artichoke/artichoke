use crate::extn::prelude::*;

pub fn init(interp: &Artichoke) -> Result<(), BootError> {
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
