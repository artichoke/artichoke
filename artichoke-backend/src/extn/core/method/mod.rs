use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Method>() {
        return Ok(());
    }
    let spec = class::Spec::new("Method", None, None)?;
    interp.def_class::<Method>(spec)?;
    let _ = interp.eval(&include_bytes!("method.rb")[..])?;
    trace!("Patched Method onto interpreter");
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Method;
