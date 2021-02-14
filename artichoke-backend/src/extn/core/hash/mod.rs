use std::ffi::CStr;

use crate::extn::prelude::*;

const HASH_CSTR: &CStr = cstr::cstr!("Hash");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Hash>() {
        return Ok(());
    }
    let spec = class::Spec::new("Hash", HASH_CSTR, None, None)?;
    interp.def_class::<Hash>(spec)?;
    let _ = interp.eval(&include_bytes!("hash.rb")[..])?;
    trace!("Patched Hash onto interpreter");
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Hash;
