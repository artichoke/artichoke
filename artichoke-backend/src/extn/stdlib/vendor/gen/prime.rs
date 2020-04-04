use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::class::Spec::new("Prime", None, None)?;
    interp.0.borrow_mut().def_class::<Prime>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Singleton", None)?;
    interp.0.borrow_mut().def_module::<Singleton>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Forwardable", None)?;
    interp.0.borrow_mut().def_module::<Forwardable>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "SingleForwardable", None)?;
    interp.0.borrow_mut().def_module::<SingleForwardable>(spec);
    
    
    
    interp.def_rb_source_file(
        b"prime.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/prime.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct Prime;


#[derive(Debug)]
pub struct Singleton;


#[derive(Debug)]
pub struct Forwardable;


#[derive(Debug)]
pub struct SingleForwardable;


