use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp
        .0
        .borrow_mut()
        .def_module::<RArtichoke>("Artichoke", None);
    Ok(())
}

#[allow(clippy::module_name_repetitions)]
pub struct RArtichoke;
