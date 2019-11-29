use artichoke_core::eval::Eval;

use crate::module;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    if interp.0.borrow().module_spec::<Enumerable>().is_some() {
        return Ok(());
    }
    let spec = module::Spec::new("Enumerable", None);
    module::Builder::for_spec(interp, &spec).define()?;
    interp.0.borrow_mut().def_module::<Enumerable>(&spec);
    interp.eval(&include_bytes!("enumerable.rb")[..])?;
    trace!("Patched Enumerable onto interpreter");
    Ok(())
}

pub struct Enumerable;
