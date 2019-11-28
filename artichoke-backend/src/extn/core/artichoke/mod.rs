use crate::def::Define;
use crate::module;
use crate::ArtichokeError;

pub fn init(interp: &crate::Artichoke) -> Result<(), ArtichokeError> {
    if interp.0.borrow().module_spec::<Artichoke>().is_some() {
        return Ok(());
    }
    let spec = module::Spec::new("Artichoke", None);
    interp.0.borrow_mut().def_module::<Artichoke>(&spec);
    spec.define(interp)?;
    trace!("Patched Artichoke onto interpreter");
    Ok(())
}

#[allow(clippy::module_name_repetitions)]
pub struct Artichoke;

pub struct Kernel;
