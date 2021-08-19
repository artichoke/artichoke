use std::ffi::CStr;

use crate::extn::prelude::*;

const PROC_CSTR: &CStr = cstr::cstr!("Proc");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Proc>() {
        return Ok(());
    }
    let spec = class::Spec::new("Proc", PROC_CSTR, None, None)?;
    interp.def_class::<Proc>(spec)?;
    interp.eval(&include_bytes!("proc.rb")[..])?;
    trace!("Patched Proc onto interpreter");
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Proc;
