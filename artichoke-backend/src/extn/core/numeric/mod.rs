use crate::extn::prelude::*;

pub fn init(interp: &Artichoke) -> InitializeResult<()> {
    if interp.0.borrow().class_spec::<Numeric>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("Numeric", None, None)?;
    interp.0.borrow_mut().def_class::<Numeric>(spec);
    let _ = interp.eval(&include_bytes!("numeric.rb")[..])?;
    trace!("Patched Numeric onto interpreter");
    Ok(())
}

pub struct Numeric;
