use crate::load::LoadSources;
use crate::ArtichokeError;
use crate::Mrb;

pub fn init(interp: &Mrb) -> Result<(), ArtichokeError> {
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
