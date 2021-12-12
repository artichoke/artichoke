use std::ffi::CStr;

use crate::extn::prelude::*;

const SET_CSTR: &CStr = cstr::cstr!("Set");
const SORTED_SET_CSTR: &CStr = cstr::cstr!("SortedSet");
static SET_RUBY_SOURCE: &[u8] = include_bytes!("vendor/set.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = class::Spec::new("Set", SET_CSTR, None, None)?;
    interp.def_class::<Set>(spec)?;
    let spec = class::Spec::new("SortedSet", SORTED_SET_CSTR, None, None)?;
    interp.def_class::<SortedSet>(spec)?;
    interp.def_rb_source_file("set.rb", SET_RUBY_SOURCE)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Set;

#[derive(Debug, Clone, Copy)]
#[allow(clippy::module_name_repetitions)]
pub struct SortedSet;
