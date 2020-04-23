use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Enumerator>() {
        return Ok(());
    }
    let spec = class::Spec::new("Enumerator", None, None)?;
    interp.def_class::<Enumerator>(spec)?;
    let _ = interp.eval(&include_bytes!("enumerator.rb")[..])?;
    let _ = interp.eval(&include_bytes!("lazy.rb")[..])?;
    trace!("Patched Enumerator onto interpreter");
    trace!("Patched Enumerator::Lazy onto interpreter");
    Ok(())
}

#[derive(Debug)]
pub struct Enumerator;
