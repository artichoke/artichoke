use crate::load::LoadSources;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp
        .0
        .borrow_mut()
        .def_class::<OpenStruct>("OpenStruct", None, None);
    interp.def_rb_source_file("ostruct.rb", include_str!("ostruct.rb"))?;
    Ok(())
}

pub struct OpenStruct;
