use log::trace;

use crate::eval::Eval;
use crate::ArtichokeError;
use crate::Mrb;

pub fn patch(interp: &Mrb) -> Result<(), ArtichokeError> {
    interp.eval(include_str!("env.rb"))?;
    trace!("Patched ENV onto interpreter");
    Ok(())
}
