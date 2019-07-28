use crate::load::LoadSources;
use crate::ArtichokeError;
use crate::Mrb;

pub fn init(interp: &Mrb) -> Result<(), ArtichokeError> {
    interp
        .borrow_mut()
        .def_class::<OpenStruct>("OpenStruct", None, None);
    interp.def_rb_source_file("ostruct.rb", include_str!("ostruct.rb"))?;
    Ok(())
}

pub struct OpenStruct;
