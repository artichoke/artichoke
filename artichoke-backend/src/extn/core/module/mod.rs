use std::ffi::CStr;

use crate::extn::prelude::*;

const MODULE_CSTR: &CStr = cstr::cstr!("Module");
static MODULE_RUBY_SOURCE: &[u8] = include_bytes!("module.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Module>() {
        return Ok(());
    }

    let spec = class::Spec::new("Module", MODULE_CSTR, None, None)?;
    interp.def_class::<Module>(spec)?;
    interp.eval(MODULE_RUBY_SOURCE)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Module;
