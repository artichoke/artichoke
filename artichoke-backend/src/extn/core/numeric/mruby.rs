use std::ffi::CStr;

use crate::extn::core::numeric::Numeric;
use crate::extn::prelude::*;

const NUMERIC_CSTR: &CStr = qed::const_cstr_from_str!("Numeric\0");
static NUMERIC_RUBY_SOURCE: &[u8] = include_bytes!("numeric.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Numeric>() {
        return Ok(());
    }
    let spec = class::Spec::new("Numeric", NUMERIC_CSTR, None, None)?;
    interp.def_class::<Numeric>(spec)?;
    interp.eval(NUMERIC_RUBY_SOURCE)?;
    Ok(())
}
