use crate::load::MrbLoadSources;
use crate::ArtichokeError;
use crate::Mrb;

pub fn init(interp: &Mrb) -> Result<(), ArtichokeError> {
    interp.borrow_mut().def_class::<Set>("Set", None, None);
    interp
        .borrow_mut()
        .def_class::<SortedSet>("SortedSet", None, None);
    interp.def_rb_source_file("set.rb", include_str!("set.rb"))?;
    Ok(())
}

pub struct Set;
#[allow(clippy::module_name_repetitions)]
pub struct SortedSet;
