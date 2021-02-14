use std::ffi::CStr;

use crate::extn::prelude::*;

const WARNING_CSTR: &CStr = cstr::cstr!("Warning");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_module_defined::<Warning>() {
        return Ok(());
    }
    let spec = module::Spec::new(interp, "Warning", WARNING_CSTR, None)?;
    module::Builder::for_spec(interp, &spec).define()?;
    interp.def_module::<Warning>(spec)?;
    let _ = interp.eval(&include_bytes!("warning.rb")[..])?;
    trace!("Patched Warning onto interpreter");
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Warning;
