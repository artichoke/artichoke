use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Module>() {
        return Ok(());
    }
    let spec = class::Spec::new("Module", None, None)?;
    interp.def_class::<Module>(spec)?;
    let _ = interp.eval(&include_bytes!("module.rb")[..])?;
    trace!("Patched Module onto interpreter");
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Module;
