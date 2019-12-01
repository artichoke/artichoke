use artichoke_core::load::LoadSources;

use crate::class;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    let spec = class::Spec::new("Set", None, None);
    interp.0.borrow_mut().def_class::<Set>(spec);
    let spec = class::Spec::new("SortedSet", None, None);
    interp.0.borrow_mut().def_class::<SortedSet>(spec);
    interp.def_rb_source_file(b"set.rb", &include_bytes!("set.rb")[..])?;
    Ok(())
}

pub struct Set;
#[allow(clippy::module_name_repetitions)]
pub struct SortedSet;
