use crate::extn::prelude::*;

#[cfg(feature = "stdlib-abbrev")]
pub mod abbrev;
#[cfg(feature = "stdlib-base64")]
pub mod base64;
#[cfg(feature = "stdlib-cmath")]
pub mod cmath;
#[cfg(feature = "stdlib-delegate")]
pub mod delegate;
#[cfg(feature = "stdlib-forwardable")]
pub mod forwardable;
#[cfg(feature = "stdlib-json")]
pub mod json;
#[cfg(feature = "stdlib-monitor")]
pub mod monitor;
#[cfg(feature = "stdlib-ostruct")]
pub mod ostruct;
#[cfg(feature = "stdlib-securerandom")]
pub mod securerandom;
#[cfg(feature = "stdlib-set")]
pub mod set;
#[cfg(feature = "stdlib-shellwords")]
pub mod shellwords;
#[cfg(feature = "stdlib-strscan")]
pub mod strscan;
#[cfg(feature = "stdlib-time")]
pub mod time;
#[cfg(feature = "stdlib-uri")]
pub mod uri;

#[allow(unused_variables)]
pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    #[cfg(feature = "stdlib-abbrev")]
    abbrev::init(interp)?;
    #[cfg(feature = "stdlib-base64")]
    base64::init(interp)?;
    #[cfg(feature = "stdlib-cmath")]
    cmath::init(interp)?;
    #[cfg(feature = "stdlib-delegate")]
    delegate::init(interp)?;
    #[cfg(feature = "stdlib-forwardable")]
    forwardable::init(interp)?;
    #[cfg(feature = "stdlib-json")]
    json::init(interp)?;
    #[cfg(feature = "stdlib-monitor")]
    monitor::init(interp)?;
    #[cfg(feature = "stdlib-ostruct")]
    ostruct::init(interp)?;
    #[cfg(feature = "stdlib-securerandom")]
    securerandom::mruby::init(interp)?;
    #[cfg(feature = "stdlib-set")]
    set::init(interp)?;
    #[cfg(feature = "stdlib-shellwords")]
    shellwords::init(interp)?;
    #[cfg(feature = "stdlib-strscan")]
    strscan::init(interp)?;
    #[cfg(feature = "stdlib-time")]
    time::init(interp)?;
    #[cfg(feature = "stdlib-uri")]
    uri::init(interp)?;

    Ok(())
}
