use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.0.borrow().class_spec::<Proc>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("Proc", None, None)?;
    interp.0.borrow_mut().def_class::<Proc>(spec);
    let _ = interp.eval(&include_bytes!("proc.rb")[..])?;
    trace!("Patched Proc onto interpreter");
    Ok(())
}

pub struct Proc;
