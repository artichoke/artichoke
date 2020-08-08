use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_module_defined::<Comparable>() {
        return Ok(());
    }
    let spec = module::Spec::new(interp, "Comparable", None)?;
    module::Builder::for_spec(interp, &spec).define()?;
    interp.def_module::<Comparable>(spec)?;
    let _ = interp.eval(&include_bytes!("comparable.rb")[..])?;
    trace!("Patched Comparable onto interpreter");
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Comparable;
