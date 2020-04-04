use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::class::Spec::new("StringIO", None, None)?;
    interp.0.borrow_mut().def_class::<StringIO>(spec);
    
    
    
    let spec = crate::class::Spec::new("ScanError", None, None)?;
    interp.0.borrow_mut().def_class::<ScanError>(spec);
    
    
    
    let spec = crate::class::Spec::new("StringScanner", None, None)?;
    interp.0.borrow_mut().def_class::<StringScanner>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Psych", None)?;
    interp.0.borrow_mut().def_module::<Psych>(spec);
    
    
    
    Ok(())
}

#[derive(Debug)]
pub struct StringIO;


#[derive(Debug)]
pub struct ScanError;


#[derive(Debug)]
pub struct StringScanner;


#[derive(Debug)]
pub struct Psych;


