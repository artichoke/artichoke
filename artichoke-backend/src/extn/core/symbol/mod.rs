use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Symbol>() {
        return Ok(());
    }
    let spec = class::Spec::new("Symbol", None, None)?;
    interp.def_class::<Symbol>(spec)?;
    let _ = interp.eval(&include_bytes!("symbol.rb")[..])?;
    trace!("Patched Symbol onto interpreter");
    Ok(())
}

#[derive(Debug)]
pub struct Symbol;
