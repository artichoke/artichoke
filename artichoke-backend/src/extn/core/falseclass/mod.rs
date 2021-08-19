use std::ffi::CStr;

use crate::extn::prelude::*;

const FALSE_CLASS_CSTR: &CStr = cstr::cstr!("FalseClass");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<FalseClass>() {
        return Ok(());
    }
    let spec = class::Spec::new("FalseClass", FALSE_CLASS_CSTR, None, None)?;
    interp.def_class::<FalseClass>(spec)?;
    interp.eval(&include_bytes!("falseclass.rb")[..])?;
    trace!("Patched FalseClass onto interpreter");
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct FalseClass;
