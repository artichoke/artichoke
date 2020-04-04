use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "Sync_m", None)?;
    interp.0.borrow_mut().def_module::<Sync_m>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Synchronizer_m", None)?;
    interp.0.borrow_mut().def_module::<Synchronizer_m>(spec);
    
    
    
    let spec = crate::class::Spec::new("Sync", None, None)?;
    interp.0.borrow_mut().def_class::<Sync>(spec);
    
    
    
    let spec = crate::class::Spec::new("Synchronizer", None, None)?;
    interp.0.borrow_mut().def_class::<Synchronizer>(spec);
    
    
    
    interp.def_rb_source_file(
        b"sync.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/sync.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct Sync_m;


#[derive(Debug)]
pub struct Synchronizer_m;


#[derive(Debug)]
pub struct Sync;


#[derive(Debug)]
pub struct Synchronizer;


