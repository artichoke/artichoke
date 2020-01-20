use artichoke_core::load::LoadSources;

use crate::extn::prelude::*;

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    let spec = class::Spec::new("Delegator", None, None)?;
    interp.0.borrow_mut().def_class::<Delegator>(spec);
    let spec = class::Spec::new("SimpleDelegator", None, None)?;
    interp.0.borrow_mut().def_class::<SimpleDelegator>(spec);
    interp.def_rb_source_file(b"delegate.rb", &include_bytes!("delegate.rb")[..])?;
    Ok(())
}

pub struct Delegator;
pub struct SimpleDelegator;
