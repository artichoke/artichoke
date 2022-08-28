use crate::extn::prelude::*;

#[cfg(feature = "stdlib-abbrev")]
pub(in crate::extn) mod abbrev;
#[cfg(feature = "stdlib-base64")]
pub(in crate::extn) mod base64;
#[cfg(feature = "stdlib-cmath")]
pub(in crate::extn) mod cmath;
#[cfg(feature = "stdlib-delegate")]
pub(in crate::extn) mod delegate;
#[cfg(feature = "stdlib-forwardable")]
pub(in crate::extn) mod forwardable;
#[cfg(feature = "stdlib-json")]
pub(in crate::extn) mod json;
#[cfg(feature = "stdlib-monitor")]
pub(in crate::extn) mod monitor;
#[cfg(feature = "stdlib-ostruct")]
pub(in crate::extn) mod ostruct;
#[cfg(feature = "stdlib-securerandom")]
pub(in crate::extn) mod securerandom;
#[cfg(feature = "stdlib-set")]
pub(in crate::extn) mod set;
#[cfg(feature = "stdlib-shellwords")]
pub(in crate::extn) mod shellwords;
#[cfg(feature = "stdlib-strscan")]
pub(in crate::extn) mod strscan;
#[cfg(feature = "stdlib-time")]
pub(in crate::extn) mod time;
#[cfg(feature = "stdlib-uri")]
pub(in crate::extn) mod uri;

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
