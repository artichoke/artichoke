use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::class::Spec::new("StringIO", None, None)?;
    interp.0.borrow_mut().def_class::<StringIO>(spec);
    
    
    
    let spec = crate::class::Spec::new("Date", None, None)?;
    interp.0.borrow_mut().def_class::<Date>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "URI", None)?;
    interp.0.borrow_mut().def_module::<URI>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "OpenURI", None)?;
    interp.0.borrow_mut().def_module::<OpenURI>(spec);
    
    
    
    let spec = crate::class::Spec::new("DateTime", None, None)?;
    interp.0.borrow_mut().def_class::<DateTime>(spec);
    
    
    
    let spec = crate::class::Spec::new("IPSocket", None, None)?;
    interp.0.borrow_mut().def_class::<IPSocket>(spec);
    
    
    
    let spec = crate::class::Spec::new("IPAddr", None, None)?;
    interp.0.borrow_mut().def_class::<IPAddr>(spec);
    
    
    
    interp.def_rb_source_file(
        b"open-uri.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/open-uri.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct StringIO;


#[derive(Debug)]
pub struct Date;


#[derive(Debug)]
pub struct URI;


#[derive(Debug)]
pub struct OpenURI;


#[derive(Debug)]
pub struct DateTime;


#[derive(Debug)]
pub struct IPSocket;


#[derive(Debug)]
pub struct IPAddr;


