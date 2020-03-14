use crate::extn::prelude::*;

pub mod abbrev;
pub mod delegate;
pub mod forwardable;
pub mod json;
pub mod monitor;
pub mod ostruct;
#[cfg(feature = "stdlib-securerandom")]
pub mod securerandom;
pub mod set;
pub mod strscan;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    abbrev::init(interp)?;
    delegate::init(interp)?;
    forwardable::init(interp)?;
    json::init(interp)?;
    monitor::init(interp)?;
    ostruct::init(interp)?;
    #[cfg(feature = "stdlib-securerandom")]
    securerandom::mruby::init(interp)?;
    set::init(interp)?;
    strscan::init(interp)?;
    uri::init(interp)?;
    Ok(())
}

pub mod uri {
    //! Ruby URI package, implemented with embedded sources from MRI 2.6.3.
    //!
    //! See `scripts/auto_import/`.

    include!(concat!(env!("OUT_DIR"), "/src/generated/uri.rs"));
}
