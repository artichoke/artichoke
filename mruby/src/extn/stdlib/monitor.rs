use crate::interpreter::Mrb;
use crate::load::MrbLoadSources;
use crate::MrbError;

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    interp.def_rb_source_file("monitor.rb", include_str!("monitor.rb"))
}
