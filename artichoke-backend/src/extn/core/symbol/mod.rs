use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.state().class_spec::<Symbol>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("Symbol", None, None)?;
    interp.state_mut().def_class::<Symbol>(spec);
    let _ = interp.eval(&include_bytes!("symbol.rb")[..])?;
    trace!("Patched Symbol onto interpreter");
    Ok(())
}

pub struct Symbol;
