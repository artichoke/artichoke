use std::ffi::CStr;

use crate::extn::prelude::*;

const METHOD_CSTR: &CStr = cstr::cstr!("Method");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Method>() {
        return Ok(());
    }
    let spec = class::Spec::new("Method", METHOD_CSTR, None, None)?;
    interp.def_class::<Method>(spec)?;
    let _ = interp.eval(&include_bytes!("method.rb")[..])?;
    trace!("Patched Method onto interpreter");
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Method;
