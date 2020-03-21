use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "Digest", None)?;
    interp.0.borrow_mut().def_module::<Digest>(spec);
    
    
    
    Ok(())
}

#[derive(Debug)]
pub struct Digest;


