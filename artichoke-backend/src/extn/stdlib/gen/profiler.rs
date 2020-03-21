use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "Profiler__", None)?;
    interp.0.borrow_mut().def_module::<Profiler__>(spec);
    
    
    
    interp.def_rb_source_file(
        b"profiler.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/profiler.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct Profiler__;


