use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "JSON", None)?;
    interp.0.borrow_mut().def_module::<JSON>(spec);
    
    
    
    let spec = crate::class::Spec::new("OpenStruct", None, None)?;
    interp.0.borrow_mut().def_class::<OpenStruct>(spec);
    
    
    
    Ok(())
}

#[derive(Debug)]
pub struct JSON;


#[derive(Debug)]
pub struct OpenStruct;


