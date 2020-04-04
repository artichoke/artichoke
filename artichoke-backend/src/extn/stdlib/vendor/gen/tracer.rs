use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    // Skipping constant SCRIPT_LINES__ with class Hash
    
    
    
    let spec = crate::class::Spec::new("Tracer", None, None)?;
    interp.0.borrow_mut().def_class::<Tracer>(spec);
    
    
    
    interp.def_rb_source_file(
        b"tracer.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/tracer.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct SCRIPT_LINES__;


#[derive(Debug)]
pub struct Tracer;


