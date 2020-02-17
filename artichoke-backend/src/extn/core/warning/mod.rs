use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.0.borrow().module_spec::<Warning>().is_some() {
        return Ok(());
    }
    let spec = module::Spec::new(interp, "Warning", None)?;
    module::Builder::for_spec(interp, &spec).define()?;
    interp.0.borrow_mut().def_module::<Warning>(spec);
    let _ = interp.eval(&include_bytes!("warning.rb")[..])?;
    trace!("Patched Warning onto interpreter");
    Ok(())
}

#[derive(Debug)]
pub struct Warning;
