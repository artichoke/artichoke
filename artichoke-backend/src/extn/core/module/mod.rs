use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.state().class_spec::<Module>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("Module", None, None)?;
    interp.state_mut().def_class::<Module>(spec);
    let _ = interp.eval(&include_bytes!("module.rb")[..])?;
    trace!("Patched Module onto interpreter");
    Ok(())
}

pub struct Module;
