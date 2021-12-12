use std::ffi::CStr;

use crate::extn::prelude::*;

const OPEN_STRUCT_CSTR: &CStr = cstr::cstr!("OpenStruct");
static OPEN_STRUCT_RUBY_SOURCE: &[u8] = include_bytes!("vendor/ostruct.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = class::Spec::new("OpenStruct", OPEN_STRUCT_CSTR, None, None)?;
    interp.def_class::<OpenStruct>(spec)?;
    interp.def_rb_source_file("ostruct.rb", OPEN_STRUCT_RUBY_SOURCE)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct OpenStruct;
