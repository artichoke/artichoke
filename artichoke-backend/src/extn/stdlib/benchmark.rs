use crate::load::LoadSources;
use crate::Artichoke;
use crate::ArtichokeError;

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    
    interp
        .borrow_mut()
        .def_class::<Benchmark>("Benchmark", None, None);
    
    interp.def_rb_source_file("benchmark.rb", include_str!("benchmark.rb"))?;
    Ok(())
}

pub struct Benchmark;

