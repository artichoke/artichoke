use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.state().class_spec::<Method>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("Method", None, None)?;
    interp.state_mut().def_class::<Method>(spec);
    let _ = interp.eval(&include_bytes!("method.rb")[..])?;
    trace!("Patched Method onto interpreter");
    Ok(())
}

pub struct Method;
