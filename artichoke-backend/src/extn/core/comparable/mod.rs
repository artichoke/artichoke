use artichoke_core::eval::Eval;

use crate::module;
use crate::{Artichoke, BootError};

pub fn init(interp: &Artichoke) -> Result<(), BootError> {
    if interp.0.borrow().module_spec::<Comparable>().is_some() {
        return Ok(());
    }
    let spec = module::Spec::new("Comparable", None)?;
    module::Builder::for_spec(interp, &spec).define()?;
    interp.0.borrow_mut().def_module::<Comparable>(spec);
    let _ = interp.eval(&include_bytes!("comparable.rb")[..])?;
    trace!("Patched Comparable onto interpreter");
    Ok(())
}

pub struct Comparable;
