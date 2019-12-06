use artichoke_core::load::LoadSources;

use crate::module;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    let spec = module::Spec::new("Abbrev", None);
    interp.0.borrow_mut().def_module::<Abbrev>(spec);
    interp.def_rb_source_file(b"abbrev.rb", &include_bytes!("abbrev.rb")[..])?;
    Ok(())
}

pub struct Abbrev;
