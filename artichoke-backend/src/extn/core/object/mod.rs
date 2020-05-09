use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Object>() {
        return Ok(());
    }
    let spec = class::Spec::new("Object", None, None)?;
    interp.def_class::<Object>(spec)?;
    let _ = interp.eval(&include_bytes!("object.rb")[..])?;
    trace!("Patched Object onto interpreter");
    Ok(())
}

#[derive(Debug)]
pub struct Object;
