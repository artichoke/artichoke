use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_module_defined::<Warning>() {
        return Ok(());
    }
    let spec = module::Spec::new(interp, "Warning", None)?;
    module::Builder::for_spec(interp, &spec).define()?;
    interp.def_module::<Warning>(spec)?;
    let _ = interp.eval(&include_bytes!("warning.rb")[..])?;
    trace!("Patched Warning onto interpreter");
    Ok(())
}

#[derive(Debug)]
pub struct Warning;
