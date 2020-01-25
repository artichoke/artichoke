use crate::extn::prelude::*;

pub fn init(interp: &Artichoke) -> InitializeResult<()> {
    if interp.0.borrow().module_spec::<Enumerable>().is_some() {
        return Ok(());
    }
    let spec = module::Spec::new(interp, "Enumerable", None)?;
    module::Builder::for_spec(interp, &spec).define()?;
    interp.0.borrow_mut().def_module::<Enumerable>(spec);
    let _ = interp.eval(&include_bytes!("enumerable.rb")[..])?;
    trace!("Patched Enumerable onto interpreter");
    Ok(())
}

pub struct Enumerable;
