use std::ffi::CStr;

use crate::extn::prelude::*;

const SHELLWORDS_CSTR: &CStr = cstr::cstr!("Shellwords");
static SHELLWORDS_RUBY_SOURCE: &[u8] = include_bytes!("vendor/shellwords.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = module::Spec::new(interp, "Shellwords", SHELLWORDS_CSTR, None)?;
    interp.def_module::<Shellwords>(spec)?;
    interp.def_rb_source_file("shellwords.rb", SHELLWORDS_RUBY_SOURCE)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Shellwords;
