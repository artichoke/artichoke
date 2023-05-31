use std::ffi::CStr;

use crate::extn::core::encoding;
use crate::extn::prelude::*;

const ENCODING_CSTR: &CStr = qed::const_cstr_from_str!("Encoding\0");
static ENCODING_RUBY_SOURCE: &[u8] = include_bytes!("encoding.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<encoding::Encoding>() {
        return Ok(());
    }

    let spec = class::Spec::new("Encoding", ENCODING_CSTR, None, None)?;
    class::Builder::for_spec(interp, &spec).define()?;
    interp.def_class::<encoding::Encoding>(spec)?;
    interp.eval(ENCODING_RUBY_SOURCE)?;

    Ok(())
}
