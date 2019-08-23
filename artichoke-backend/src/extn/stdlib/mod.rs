use crate::{Artichoke, ArtichokeError};

pub mod delegate;
pub mod forwardable;
pub mod json;
pub mod monitor;
pub mod ostruct;
pub mod set;
pub mod strscan;
mod stubs;

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    base64::init(interp)?;
    stubs::init(interp)?;
    delegate::init(interp)?;
    forwardable::init(interp)?;
    json::init(interp)?;
    monitor::init(interp)?;
    ostruct::init(interp)?;
    set::init(interp)?;
    strscan::init(interp)?;
    uri::init(interp)?;
    Ok(())
}

pub mod base64 {
    //! Ruby Base64 package, implemented with embedded sources from MRI 2.6.3.
    // See scripts/auto_import/.
    include!(concat!(env!("OUT_DIR"), "/src/generated/base64.rs"));
}

pub mod uri {
    //! Ruby URI package, implemented with embedded sources from MRI 2.6.3.
    // See scripts/auto_import/.
    include!(concat!(env!("OUT_DIR"), "/src/generated/uri.rs"));
}
