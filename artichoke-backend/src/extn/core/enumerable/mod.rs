use std::ffi::CStr;

use crate::extn::prelude::*;

const ENUMERABLE_CSTR: &CStr = cstr::cstr!("Enumerable");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_module_defined::<Enumerable>() {
        return Ok(());
    }
    let spec = module::Spec::new(interp, "Enumerable", ENUMERABLE_CSTR, None)?;
    module::Builder::for_spec(interp, &spec).define()?;
    interp.def_module::<Enumerable>(spec)?;
    interp.eval(&include_bytes!("enumerable.rb")[..])?;
    trace!("Patched Enumerable onto interpreter");
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Enumerable;
