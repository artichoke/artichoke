use std::ffi::CStr;

use crate::extn::prelude::*;

const METHOD_CSTR: &CStr = qed::const_cstr_from_str!("Method\0");
static METHOD_RUBY_SOURCE: &[u8] = include_bytes!("method.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Method>() {
        return Ok(());
    }

    let spec = class::Spec::new("Method", METHOD_CSTR, None, None)?;
    interp.def_class::<Method>(spec)?;
    interp.eval(METHOD_RUBY_SOURCE)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Method;
