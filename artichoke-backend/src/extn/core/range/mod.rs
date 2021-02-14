use std::ffi::CStr;

use crate::extn::prelude::*;

const RANGE_CSTR: &CStr = cstr::cstr!("Range");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Range>() {
        return Ok(());
    }
    let spec = class::Spec::new("Range", RANGE_CSTR, None, None)?;
    interp.def_class::<Range>(spec)?;
    let _ = interp.eval(&include_bytes!("range.rb")[..])?;
    trace!("Patched Range onto interpreter");
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Range;
