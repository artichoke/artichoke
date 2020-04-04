use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "TSort", None)?;
    interp.0.borrow_mut().def_module::<TSort>(spec);
    
    
    
    interp.def_rb_source_file(
        b"tsort.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/tsort.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct TSort;


