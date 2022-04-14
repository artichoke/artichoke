use std::ffi::CStr;

use crate::extn::prelude::*;

const TRUE_CLASS_CSTR: &CStr = qed::const_cstr_from_str!("TrueClass\0");
static TRUE_CLASS_RUBY_SOURCE: &[u8] = include_bytes!("trueclass.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<TrueClass>() {
        return Ok(());
    }

    let spec = class::Spec::new("TrueClass", TRUE_CLASS_CSTR, None, None)?;
    interp.def_class::<TrueClass>(spec)?;
    interp.eval(TRUE_CLASS_RUBY_SOURCE)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct TrueClass;
