use crate::extn::prelude::*;

pub fn init(interp: &Artichoke) -> InitializeResult<()> {
    if interp.0.borrow().class_spec::<Hash>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("Hash", None, None)?;
    interp.0.borrow_mut().def_class::<Hash>(spec);
    let _ = interp.eval(&include_bytes!("hash.rb")[..])?;
    trace!("Patched Hash onto interpreter");
    Ok(())
}

pub struct Hash;
