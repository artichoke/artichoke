use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::class::Spec::new("SDBM", None, None)?;
    interp.0.borrow_mut().def_class::<SDBM>(spec);
    
    
    
    let spec = crate::class::Spec::new("SDBMError", None, None)?;
    interp.0.borrow_mut().def_class::<SDBMError>(spec);
    
    
    
    Ok(())
}

#[derive(Debug)]
pub struct SDBM;


#[derive(Debug)]
pub struct SDBMError;


