use std::ffi::CStr;

use crate::extn::prelude::*;

const ENUMERATOR_CSTR: &CStr = cstr::cstr!("Enumerator");
static ENUMERATOR_RUBY_SOURCE: &[u8] = include_bytes!("enumerator.rb");
static ENUMERATOR_LAZY_RUBY_SOURCE: &[u8] = include_bytes!("lazy.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Enumerator>() {
        return Ok(());
    }

    let spec = class::Spec::new("Enumerator", ENUMERATOR_CSTR, None, None)?;
    interp.def_class::<Enumerator>(spec)?;
    interp.eval(ENUMERATOR_RUBY_SOURCE)?;
    interp.eval(ENUMERATOR_LAZY_RUBY_SOURCE)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Enumerator;
