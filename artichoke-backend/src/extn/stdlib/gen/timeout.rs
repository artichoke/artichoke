use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "Timeout", None)?;
    interp.0.borrow_mut().def_module::<Timeout>(spec);
    
    
    
    let spec = crate::class::Spec::new("TimeoutError", None, None)?;
    interp.0.borrow_mut().def_class::<TimeoutError>(spec);
    
    
    
    interp.def_rb_source_file(
        b"timeout.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/timeout.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct Timeout;


#[derive(Debug)]
pub struct TimeoutError;


