use artichoke_core::eval::Eval;

use crate::class;
use crate::{Artichoke, BootError};

pub fn init(interp: &Artichoke) -> Result<(), BootError> {
    if interp.0.borrow().class_spec::<Hash>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("Hash", None, None)?;
    interp.0.borrow_mut().def_class::<Hash>(spec);
    let _ = interp.eval(&include_bytes!("hash.rb")[..])?;
    trace!("Patched Hash onto interpreter");
    Ok(())
}

pub struct Hash;
