use std::ffi::CStr;

use crate::extn::prelude::*;

const OBJECT_CSTR: &CStr = cstr::cstr!("Object");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Object>() {
        return Ok(());
    }
    let spec = class::Spec::new("Object", OBJECT_CSTR, None, None)?;
    interp.def_class::<Object>(spec)?;
    let _ = interp.eval(&include_bytes!("object.rb")[..])?;
    trace!("Patched Object onto interpreter");
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Object;
