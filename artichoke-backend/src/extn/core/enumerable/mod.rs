use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.state().module_spec::<Enumerable>().is_some() {
        return Ok(());
    }
    let spec = module::Spec::new("Enumerable", None)?;
    module::Builder::for_spec(interp, &spec).define()?;
    interp.state_mut().def_module::<Enumerable>(spec);
    let _ = interp.eval(&include_bytes!("enumerable.rb")[..])?;
    trace!("Patched Enumerable onto interpreter");
    Ok(())
}

pub struct Enumerable;
