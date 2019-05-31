use crate::interpreter::Mrb;
use crate::load::MrbLoadSources;
use crate::MrbError;

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    interp
        .borrow_mut()
        .def_class::<Monitor>("Monitor", None, None);
    interp.def_rb_source_file("monitor.rb", include_str!("monitor.rb"))
}

pub struct Monitor;
