use std::ffi::CStr;

use crate::extn::prelude::*;

const NIL_CLASS_CSTR: &CStr = cstr::cstr!("NilClass");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<NilClass>() {
        return Ok(());
    }
    let spec = class::Spec::new("NilClass", NIL_CLASS_CSTR, None, None)?;
    interp.def_class::<NilClass>(spec)?;
    let _ = interp.eval(&include_bytes!("nilclass.rb")[..])?;
    trace!("Patched NilClass onto interpreter");
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct NilClass;
