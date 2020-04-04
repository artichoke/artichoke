use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::class::Spec::new("PStore", None, None)?;
    interp.0.borrow_mut().def_class::<PStore>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Digest", None)?;
    interp.0.borrow_mut().def_module::<Digest>(spec);
    
    
    
    interp.def_rb_source_file(
        b"pstore.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/pstore.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct PStore;


#[derive(Debug)]
pub struct Digest;


