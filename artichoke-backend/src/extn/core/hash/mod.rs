use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Hash>() {
        return Ok(());
    }
    let spec = class::Spec::new("Hash", None, None)?;
    interp.def_class::<Hash>(spec)?;
    let _ = interp.eval(&include_bytes!("hash.rb")[..])?;
    trace!("Patched Hash onto interpreter");
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Hash;
