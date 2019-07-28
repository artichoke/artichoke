use log::trace;

use crate::eval::Eval;
use crate::Artichoke;
use crate::ArtichokeError;

pub fn patch(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp.eval(include_str!("env.rb"))?;
    trace!("Patched ENV onto interpreter");
    Ok(())
}
