use artichoke_core::load::LoadSources;

use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp
        .0
        .borrow_mut()
        .def_class::<Delegator>("Delegator", None, None);
    interp
        .0
        .borrow_mut()
        .def_class::<SimpleDelegator>("SimpleDelegator", None, None);
    interp.def_rb_source_file(b"delegate.rb", &include_bytes!("delegate.rb")[..])?;
    Ok(())
}

pub struct Delegator;
pub struct SimpleDelegator;
