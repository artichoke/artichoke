use std::ffi::CStr;

use crate::extn::prelude::*;

const ENUMERATOR_CSTR: &CStr = cstr::cstr!("Enumerator");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Enumerator>() {
        return Ok(());
    }
    let spec = class::Spec::new("Enumerator", ENUMERATOR_CSTR, None, None)?;
    interp.def_class::<Enumerator>(spec)?;
    let _ = interp.eval(&include_bytes!("enumerator.rb")[..])?;
    let _ = interp.eval(&include_bytes!("lazy.rb")[..])?;
    trace!("Patched Enumerator onto interpreter");
    trace!("Patched Enumerator::Lazy onto interpreter");
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Enumerator;
