use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.state().class_spec::<Range>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("Range", None, None)?;
    interp.state_mut().def_class::<Range>(spec);
    let _ = interp.eval(&include_bytes!("range.rb")[..])?;
    trace!("Patched Range onto interpreter");
    Ok(())
}

pub struct Range;
