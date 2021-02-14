use std::ffi::CStr;

use crate::extn::prelude::*;

const DELEGATOR_CSTR: &CStr = cstr::cstr!("Delegator");
const SIMPLE_DELEGATOR_CSTR: &CStr = cstr::cstr!("SimpleDelegator");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = class::Spec::new("Delegator", DELEGATOR_CSTR, None, None)?;
    interp.def_class::<Delegator>(spec)?;
    let spec = class::Spec::new("SimpleDelegator", SIMPLE_DELEGATOR_CSTR, None, None)?;
    interp.def_class::<SimpleDelegator>(spec)?;
    interp.def_rb_source_file("delegate.rb", &include_bytes!("vendor/delegate.rb")[..])?;
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Delegator;

#[derive(Debug, Clone, Copy)]
pub struct SimpleDelegator;
