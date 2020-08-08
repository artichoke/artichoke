use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Proc>() {
        return Ok(());
    }
    let spec = class::Spec::new("Proc", None, None)?;
    interp.def_class::<Proc>(spec)?;
    let _ = interp.eval(&include_bytes!("proc.rb")[..])?;
    trace!("Patched Proc onto interpreter");
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Proc;
