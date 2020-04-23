use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Range>() {
        return Ok(());
    }
    let spec = class::Spec::new("Range", None, None)?;
    interp.def_class::<Range>(spec)?;
    let _ = interp.eval(&include_bytes!("range.rb")[..])?;
    trace!("Patched Range onto interpreter");
    Ok(())
}

#[derive(Debug)]
pub struct Range;
