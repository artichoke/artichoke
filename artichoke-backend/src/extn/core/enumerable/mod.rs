use artichoke_core::eval::Eval;

use crate::module;
use crate::{Artichoke, BootError};

pub fn init(interp: &Artichoke) -> Result<(), BootError> {
    if interp.0.borrow().module_spec::<Enumerable>().is_some() {
        return Ok(());
    }
    let spec = module::Spec::new("Enumerable", None)?;
    module::Builder::for_spec(interp, &spec).define()?;
    interp.0.borrow_mut().def_module::<Enumerable>(spec);
    let _ = interp.eval(&include_bytes!("enumerable.rb")[..])?;
    trace!("Patched Enumerable onto interpreter");
    Ok(())
}

pub struct Enumerable;
