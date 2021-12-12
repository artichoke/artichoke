use std::ffi::CStr;

use crate::extn::prelude::*;

const CMATH_CSTR: &CStr = cstr::cstr!("CMath");
static CMATH_RUBY_SOURCE: &[u8] = include_bytes!("vendor/cmath.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = module::Spec::new(interp, "CMath", CMATH_CSTR, None)?;
    interp.def_module::<CMath>(spec)?;
    interp.def_rb_source_file("cmath.rb", CMATH_RUBY_SOURCE)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct CMath;
