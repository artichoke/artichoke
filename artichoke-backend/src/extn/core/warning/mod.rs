use std::ffi::CStr;

use crate::extn::prelude::*;

const WARNING_CSTR: &CStr = cstr::cstr!("Warning");
static WARNING_RUBY_SOURCE: &[u8] = include_bytes!("warning.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_module_defined::<Warning>() {
        return Ok(());
    }

    let spec = module::Spec::new(interp, "Warning", WARNING_CSTR, None)?;
    module::Builder::for_spec(interp, &spec).define()?;
    interp.def_module::<Warning>(spec)?;
    interp.eval(WARNING_RUBY_SOURCE)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Warning;
