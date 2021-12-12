use std::ffi::CStr;

use crate::extn::prelude::*;

const ARTICHOKE_CSTR: &CStr = cstr::cstr!("Artichoke");

pub fn init(interp: &mut crate::Artichoke) -> InitializeResult<()> {
    if interp.is_module_defined::<Artichoke>() {
        return Ok(());
    }

    let spec = module::Spec::new(interp, "Artichoke", ARTICHOKE_CSTR, None)?;
    module::Builder::for_spec(interp, &spec).define()?;
    interp.def_module::<Artichoke>(spec)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Artichoke;

#[derive(Debug, Clone, Copy)]
pub struct Kernel;
