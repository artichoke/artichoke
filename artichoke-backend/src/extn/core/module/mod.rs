use std::ffi::CStr;

use crate::extn::prelude::*;

const MODULE_CSTR: &CStr = cstr::cstr!("Module");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Module>() {
        return Ok(());
    }
    let spec = class::Spec::new("Module", MODULE_CSTR, None, None)?;
    interp.def_class::<Module>(spec)?;
    interp.eval(&include_bytes!("module.rb")[..])?;
    trace!("Patched Module onto interpreter");
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Module;
