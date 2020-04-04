use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::class::Spec::new("ThreadsWait", None, None)?;
    interp.0.borrow_mut().def_class::<ThreadsWait>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Exception2MessageMapper", None)?;
    interp.0.borrow_mut().def_module::<Exception2MessageMapper>(spec);
    
    
    
    let spec = crate::class::Spec::new("ThWait", None, None)?;
    interp.0.borrow_mut().def_class::<ThWait>(spec);
    
    
    
    interp.def_rb_source_file(
        b"thwait.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/thwait.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct ThreadsWait;


#[derive(Debug)]
pub struct Exception2MessageMapper;


#[derive(Debug)]
pub struct ThWait;


