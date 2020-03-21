use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "Readline", None)?;
    interp.0.borrow_mut().def_module::<Readline>(spec);
    
    
    
    Ok(())
}

#[derive(Debug)]
pub struct Readline;


