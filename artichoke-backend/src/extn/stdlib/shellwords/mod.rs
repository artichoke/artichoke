use std::ffi::CStr;

use crate::extn::prelude::*;

const SHELLWORDS_CSTR: &CStr = cstr::cstr!("Shellwords");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = module::Spec::new(interp, "Shellwords", SHELLWORDS_CSTR, None)?;
    interp.def_module::<Shellwords>(spec)?;
    interp.def_rb_source_file("shellwords.rb", &include_bytes!("vendor/shellwords.rb")[..])?;
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Shellwords;
