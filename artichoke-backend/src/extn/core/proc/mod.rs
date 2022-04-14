use std::ffi::CStr;

use crate::extn::prelude::*;

const PROC_CSTR: &CStr = qed::const_cstr_from_str!("Proc\0");
static PROC_RUBY_SOURCE: &[u8] = include_bytes!("proc.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Proc>() {
        return Ok(());
    }

    let spec = class::Spec::new("Proc", PROC_CSTR, None, None)?;
    interp.def_class::<Proc>(spec)?;
    interp.eval(PROC_RUBY_SOURCE)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Proc;
