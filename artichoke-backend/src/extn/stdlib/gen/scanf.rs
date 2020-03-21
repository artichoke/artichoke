use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "Scanf", None)?;
    interp.0.borrow_mut().def_module::<Scanf>(spec);
    
    
    
    interp.def_rb_source_file(
        b"scanf.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/scanf.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct Scanf;


