use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::class::Spec::new("Ripper", None, None)?;
    interp.0.borrow_mut().def_class::<Ripper>(spec);
    
    
    
    Ok(())
}

#[derive(Debug)]
pub struct Ripper;


