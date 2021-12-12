use std::ffi::CStr;

use crate::extn::prelude::*;

const BASE64_CSTR: &CStr = cstr::cstr!("Base64");
static BASE64_RUBY_SOURCE: &[u8] = include_bytes!("vendor/base64.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = crate::module::Spec::new(interp, "Base64", BASE64_CSTR, None)?;
    interp.def_module::<Base64>(spec)?;
    interp.def_rb_source_file("base64.rb", BASE64_RUBY_SOURCE)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Base64;
