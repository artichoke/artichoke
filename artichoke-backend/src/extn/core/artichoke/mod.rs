use crate::extn::prelude::*;

pub fn init(interp: &mut crate::Artichoke) -> InitializeResult<()> {
    if interp.is_module_defined::<Artichoke>() {
        return Ok(());
    }
    let spec = module::Spec::new(interp, "Artichoke", None)?;
    module::Builder::for_spec(interp, &spec).define()?;
    interp.def_module::<Artichoke>(spec)?;
    trace!("Patched Artichoke onto interpreter");
    Ok(())
}

#[derive(Debug)]
pub struct Artichoke;

#[derive(Debug)]
pub struct Kernel;
