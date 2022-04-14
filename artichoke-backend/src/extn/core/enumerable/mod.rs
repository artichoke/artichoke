use std::ffi::CStr;

use crate::extn::prelude::*;

const ENUMERABLE_CSTR: &CStr = qed::const_cstr_from_str!("Enumerable\0");
static ENUMERABLE_RUBY_SOURCE: &[u8] = include_bytes!("enumerable.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_module_defined::<Enumerable>() {
        return Ok(());
    }

    let spec = module::Spec::new(interp, "Enumerable", ENUMERABLE_CSTR, None)?;
    module::Builder::for_spec(interp, &spec).define()?;
    interp.def_module::<Enumerable>(spec)?;
    interp.eval(ENUMERABLE_RUBY_SOURCE)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Enumerable;
