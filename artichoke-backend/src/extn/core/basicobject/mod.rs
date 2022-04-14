use std::ffi::CStr;

use crate::extn::prelude::*;

const BASIC_OBJECT_CSTR: &CStr = qed::const_cstr_from_str!("BasicObject\0");
static BASIC_OBJECT_RUBY_SOURCE: &[u8] = include_bytes!("basicobject.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<BasicObject>() {
        return Ok(());
    }

    let spec = class::Spec::new("BasicObject", BASIC_OBJECT_CSTR, None, None)?;
    interp.def_class::<BasicObject>(spec)?;
    interp.eval(BASIC_OBJECT_RUBY_SOURCE)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct BasicObject;
