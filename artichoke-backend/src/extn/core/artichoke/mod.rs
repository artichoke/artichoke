use crate::extn::prelude::*;

pub fn init(interp: &crate::Artichoke) -> InitializeResult<()> {
    if interp.state().module_spec::<Artichoke>().is_some() {
        return Ok(());
    }
    let spec = module::Spec::new("Artichoke", None)?;
    module::Builder::for_spec(interp, &spec).define()?;
    interp.state_mut().def_module::<Artichoke>(spec);
    trace!("Patched Artichoke onto interpreter");
    Ok(())
}

pub struct Artichoke;

pub struct Kernel;
