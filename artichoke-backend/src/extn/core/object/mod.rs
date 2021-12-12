use std::ffi::CStr;

use crate::extn::prelude::*;

const OBJECT_CSTR: &CStr = cstr::cstr!("Object");
static OBJECT_RUBY_SOURCE: &[u8] = include_bytes!("object.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Object>() {
        return Ok(());
    }

    let spec = class::Spec::new("Object", OBJECT_CSTR, None, None)?;
    interp.def_class::<Object>(spec)?;
    interp.eval(OBJECT_RUBY_SOURCE)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Object;
