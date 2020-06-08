use crate::extn::prelude::*;

pub mod abbrev;
pub mod base64;
pub mod cmath;
pub mod delegate;
pub mod forwardable;
pub mod json;
pub mod monitor;
pub mod ostruct;
#[cfg(feature = "stdlib-securerandom")]
pub mod securerandom;
pub mod set;
pub mod shellwords;
pub mod strscan;
pub mod time;
pub mod uri;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    abbrev::init(interp)?;
    base64::init(interp)?;
    cmath::init(interp)?;
    delegate::init(interp)?;
    forwardable::init(interp)?;
    json::init(interp)?;
    monitor::init(interp)?;
    ostruct::init(interp)?;
    #[cfg(feature = "stdlib-securerandom")]
    securerandom::mruby::init(interp)?;
    set::init(interp)?;
    shellwords::init(interp)?;
    strscan::init(interp)?;
    time::init(interp)?;
    uri::init(interp)?;
    Ok(())
}
