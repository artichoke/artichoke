use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.state().class_spec::<Numeric>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("Numeric", None, None)?;
    interp.state_mut().def_class::<Numeric>(spec);
    let _ = interp.eval(&include_bytes!("numeric.rb")[..])?;
    trace!("Patched Numeric onto interpreter");
    Ok(())
}

pub struct Numeric;
