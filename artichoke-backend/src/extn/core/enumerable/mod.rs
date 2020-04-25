use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_module_defined::<Enumerable>() {
        return Ok(());
    }
    let spec = module::Spec::new(interp, "Enumerable", None)?;
    module::Builder::for_spec(interp, &spec).define()?;
    interp.def_module::<Enumerable>(spec)?;
    let _ = interp.eval(&include_bytes!("enumerable.rb")[..])?;
    trace!("Patched Enumerable onto interpreter");
    Ok(())
}

#[derive(Debug)]
pub struct Enumerable;
