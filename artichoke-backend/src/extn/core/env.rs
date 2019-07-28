use log::trace;

use crate::eval::MrbEval;
use crate::Mrb;
use crate::MrbError;

pub fn patch(interp: &Mrb) -> Result<(), MrbError> {
    interp.eval(include_str!("env.rb"))?;
    trace!("Patched ENV onto interpreter");
    Ok(())
}
