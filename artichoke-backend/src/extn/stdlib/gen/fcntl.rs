use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "Fcntl", None)?;
    interp.0.borrow_mut().def_module::<Fcntl>(spec);
    
    
    
    Ok(())
}

#[derive(Debug)]
pub struct Fcntl;


