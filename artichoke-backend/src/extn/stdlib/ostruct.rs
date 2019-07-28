use crate::load::MrbLoadSources;
use crate::Mrb;
use crate::MrbError;

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    interp
        .borrow_mut()
        .def_class::<OpenStruct>("OpenStruct", None, None);
    interp.def_rb_source_file("ostruct.rb", include_str!("ostruct.rb"))?;
    Ok(())
}

pub struct OpenStruct;
