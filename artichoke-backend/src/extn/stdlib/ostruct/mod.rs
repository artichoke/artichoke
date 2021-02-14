use std::ffi::CStr;

use crate::extn::prelude::*;

const OPEN_STRUCT_CSTR: &CStr = cstr::cstr!("OpenStruct");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = class::Spec::new("OpenStruct", OPEN_STRUCT_CSTR, None, None)?;
    interp.def_class::<OpenStruct>(spec)?;
    interp.def_rb_source_file("ostruct.rb", &include_bytes!("vendor/ostruct.rb")[..])?;
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct OpenStruct;
