use crate::{ArtichokeError, Mrb};

pub mod delegate;
pub mod forwardable;
pub mod json;
pub mod monitor;
pub mod ostruct;
pub mod set;
pub mod strscan;
mod stubs;
pub mod uri;

pub fn patch(interp: &Mrb) -> Result<(), ArtichokeError> {
    stubs::patch(interp)?;
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
