use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "Benchmark", None)?;
    interp.0.borrow_mut().def_module::<Benchmark>(spec);
    
    
    
    interp.def_rb_source_file(
        b"benchmark.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/benchmark.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct Benchmark;


