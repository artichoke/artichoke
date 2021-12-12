use std::ffi::CStr;

use crate::extn::prelude::*;

const NIL_CLASS_CSTR: &CStr = cstr::cstr!("NilClass");
static NIL_CLASS_RUBY_SOURCE: &[u8] = include_bytes!("nilclass.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<NilClass>() {
        return Ok(());
    }

    let spec = class::Spec::new("NilClass", NIL_CLASS_CSTR, None, None)?;
    interp.def_class::<NilClass>(spec)?;
    interp.eval(NIL_CLASS_RUBY_SOURCE)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct NilClass;
