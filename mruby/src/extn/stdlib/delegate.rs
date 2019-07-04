use crate::load::MrbLoadSources;
use crate::Mrb;
use crate::MrbError;

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    interp
        .borrow_mut()
        .def_class::<Delegator>("Delegator", None, None);
    interp
        .borrow_mut()
        .def_class::<SimpleDelegator>("SimpleDelegator", None, None);
    interp.def_rb_source_file("delegate.rb", include_str!("delegate.rb"))?;
    Ok(())
}

pub struct Delegator;
pub struct SimpleDelegator;
