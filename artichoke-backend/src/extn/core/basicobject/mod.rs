use std::ffi::CStr;

use crate::extn::prelude::*;

const BASIC_OBJECT_CSTR: &CStr = cstr::cstr!("BasicObject");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<BasicObject>() {
        return Ok(());
    }
    let spec = class::Spec::new("BasicObject", BASIC_OBJECT_CSTR, None, None)?;
    interp.def_class::<BasicObject>(spec)?;
    let _ = interp.eval(&include_bytes!("basicobject.rb")[..])?;
    trace!("Patched BasicObject onto interpreter");
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct BasicObject;
