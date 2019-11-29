use artichoke_core::eval::Eval;

use crate::module;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    if interp.0.borrow().module_spec::<Warning>().is_some() {
        return Ok(());
    }
    let spec = module::Spec::new("Warning", None);
    module::Builder::for_spec(interp, &spec).define()?;
    interp.0.borrow_mut().def_module::<Warning>(&spec);
    interp.eval(&include_bytes!("warning.rb")[..])?;
    trace!("Patched Warning onto interpreter");
    Ok(())
}

pub struct Warning;
