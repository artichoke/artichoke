use crate::interpreter::Mrb;
use crate::load::MrbLoadSources;
use crate::MrbError;

const SRC: &str = include_str!("monitor.rb");

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    interp.def_rb_source_file("monitor.rb", SRC)
}
