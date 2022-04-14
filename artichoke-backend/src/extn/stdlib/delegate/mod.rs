use std::ffi::CStr;

use crate::extn::prelude::*;

const DELEGATOR_CSTR: &CStr = qed::const_cstr_from_str!("Delegator\0");
const SIMPLE_DELEGATOR_CSTR: &CStr = qed::const_cstr_from_str!("SimpleDelegator\0");
static DELEGATE_RUBY_SOURCE: &[u8] = include_bytes!("vendor/delegate.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = class::Spec::new("Delegator", DELEGATOR_CSTR, None, None)?;
    interp.def_class::<Delegator>(spec)?;
    let spec = class::Spec::new("SimpleDelegator", SIMPLE_DELEGATOR_CSTR, None, None)?;
    interp.def_class::<SimpleDelegator>(spec)?;
    interp.def_rb_source_file("delegate.rb", DELEGATE_RUBY_SOURCE)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Delegator;

#[derive(Debug, Clone, Copy)]
pub struct SimpleDelegator;
