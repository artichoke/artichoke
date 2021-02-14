use std::ffi::CStr;

use crate::extn::prelude::*;

const CMATH_CSTR: &CStr = cstr::cstr!("CMath");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = module::Spec::new(interp, "CMath", CMATH_CSTR, None)?;
    interp.def_module::<CMath>(spec)?;
    interp.def_rb_source_file("cmath.rb", &include_bytes!("vendor/cmath.rb")[..])?;
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct CMath;
