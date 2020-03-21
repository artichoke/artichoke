use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::class::Spec::new("WeakRef", None, None)?;
    interp.0.borrow_mut().def_class::<WeakRef>(spec);
    
    
    
    let spec = crate::class::Spec::new("Delegator", None, None)?;
    interp.0.borrow_mut().def_class::<Delegator>(spec);
    
    
    
    let spec = crate::class::Spec::new("SimpleDelegator", None, None)?;
    interp.0.borrow_mut().def_class::<SimpleDelegator>(spec);
    
    
    
    interp.def_rb_source_file(
        b"weakref.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/weakref.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct WeakRef;


#[derive(Debug)]
pub struct Delegator;


#[derive(Debug)]
pub struct SimpleDelegator;


