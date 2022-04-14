use std::ffi::CStr;

use crate::extn::prelude::*;

const FALSE_CLASS_CSTR: &CStr = qed::const_cstr_from_str!("FalseClass\0");
static FALSE_CLASS_RUBY_SOURCE: &[u8] = include_bytes!("falseclass.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<FalseClass>() {
        return Ok(());
    }

    let spec = class::Spec::new("FalseClass", FALSE_CLASS_CSTR, None, None)?;
    interp.def_class::<FalseClass>(spec)?;
    interp.eval(FALSE_CLASS_RUBY_SOURCE)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct FalseClass;
