use crate::load::LoadSources;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp.0.borrow_mut().def_class::<Set>("Set", None, None);
    interp
        .0
        .borrow_mut()
        .def_class::<SortedSet>("SortedSet", None, None);
    interp.def_rb_source_file("set.rb", include_str!("set.rb"))?;
    Ok(())
}

pub struct Set;
#[allow(clippy::module_name_repetitions)]
pub struct SortedSet;
