use artichoke_core::eval::Eval;

use crate::module;
use crate::{Artichoke, BootError};

pub fn init(interp: &Artichoke) -> Result<(), BootError> {
    if interp.0.borrow().module_spec::<Warning>().is_some() {
        return Ok(());
    }
    let spec = module::Spec::new("Warning", None)?;
    module::Builder::for_spec(interp, &spec).define()?;
    interp.0.borrow_mut().def_module::<Warning>(spec);
    let _ = interp.eval(&include_bytes!("warning.rb")[..])?;
    trace!("Patched Warning onto interpreter");
    Ok(())
}

pub struct Warning;
