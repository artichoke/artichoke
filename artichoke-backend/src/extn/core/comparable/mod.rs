use std::ffi::CStr;

use crate::extn::prelude::*;

const COMPARABLE_CSTR: &CStr = qed::const_cstr_from_str!("Comparable\0");
static COMPARABLE_RUBY_SOURCE: &[u8] = include_bytes!("comparable.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_module_defined::<Comparable>() {
        return Ok(());
    }

    let spec = module::Spec::new(interp, "Comparable", COMPARABLE_CSTR, None)?;
    module::Builder::for_spec(interp, &spec).define()?;
    interp.def_module::<Comparable>(spec)?;
    interp.eval(COMPARABLE_RUBY_SOURCE)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Comparable;
