use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.state().class_spec::<Hash>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("Hash", None, None)?;
    interp.state_mut().def_class::<Hash>(spec);
    let _ = interp.eval(&include_bytes!("hash.rb")[..])?;
    trace!("Patched Hash onto interpreter");
    Ok(())
}

pub struct Hash;
