use std::ffi::CStr;

use crate::extn::prelude::*;

const RANGE_CSTR: &CStr = qed::const_cstr_from_str!("Range\0");
static RANGE_RUBY_SOURCE: &[u8] = include_bytes!("range.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Range>() {
        return Ok(());
    }

    let spec = class::Spec::new("Range", RANGE_CSTR, None, None)?;
    interp.def_class::<Range>(spec)?;
    interp.eval(RANGE_RUBY_SOURCE)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Range;
