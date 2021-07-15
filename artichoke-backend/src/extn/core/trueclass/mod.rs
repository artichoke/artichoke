use std::ffi::CStr;

use crate::extn::prelude::*;

const TRUE_CLASS_CSTR: &CStr = cstr::cstr!("TrueClass");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<TrueClass>() {
        return Ok(());
    }
    let spec = class::Spec::new("TrueClass", TRUE_CLASS_CSTR, None, None)?;
    interp.def_class::<TrueClass>(spec)?;
    let _ = interp.eval(&include_bytes!("trueclass.rb")[..])?;
    trace!("Patched TrueClass onto interpreter");
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct TrueClass;
