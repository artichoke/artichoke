use mruby::interpreter::{Interpreter, Mrb};
use mruby::load::MrbLoadSources;
use mruby::MrbError;

use crate::sources::foolsgold;

pub mod prefork;
pub mod shared_nothing;

fn interpreter() -> Result<Mrb, MrbError> {
    let mut interp = Interpreter::create()?;
    nemesis::init(&mut interp)?;
    interp.def_file_for_type::<_, foolsgold::Lib>("foolsgold")?;
    Ok(interp)
}
