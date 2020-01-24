use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.state().module_spec::<Comparable>().is_some() {
        return Ok(());
    }
    let spec = module::Spec::new(interp, "Comparable", None)?;
    module::Builder::for_spec(interp, &spec).define()?;
    interp.state_mut().def_module::<Comparable>(spec);
    let _ = interp.eval(&include_bytes!("comparable.rb")[..])?;
    trace!("Patched Comparable onto interpreter");
    Ok(())
}

pub struct Comparable;
