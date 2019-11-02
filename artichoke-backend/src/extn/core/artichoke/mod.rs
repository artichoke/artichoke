use crate::def::Define;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    let artichoke = interp
        .0
        .borrow_mut()
        .def_module::<RArtichoke>("Artichoke", None);
    artichoke.borrow().define(interp)?;
    Ok(())
}

#[allow(clippy::module_name_repetitions)]
pub struct RArtichoke;
