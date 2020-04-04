use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::class::Spec::new("CGI", None, None)?;
    interp.0.borrow_mut().def_class::<CGI>(spec);
    
    
    
    let spec = crate::class::Spec::new("ScanError", None, None)?;
    interp.0.borrow_mut().def_class::<ScanError>(spec);
    
    
    
    let spec = crate::class::Spec::new("ERB", None, None)?;
    interp.0.borrow_mut().def_class::<ERB>(spec);
    
    
    
    let spec = crate::class::Spec::new("StringScanner", None, None)?;
    interp.0.borrow_mut().def_class::<StringScanner>(spec);
    
    
    
    interp.def_rb_source_file(
        b"erb.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/erb.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct CGI;


#[derive(Debug)]
pub struct ScanError;


#[derive(Debug)]
pub struct ERB;


#[derive(Debug)]
pub struct StringScanner;


