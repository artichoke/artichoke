use crate::interpreter::Mrb;
use crate::load::MrbLoadSources;
use crate::MrbError;

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    interp
        .borrow_mut()
        .def_class::<Thread>("Thread", None, None);
    interp
        .borrow_mut()
        .def_class::<Mutex>("Mutex", None, None);
    interp.def_rb_source_file("thread.rb", include_str!("thread.rb"))
}

pub struct Thread;
pub struct Mutex;
