use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "Abbrev", None)?;
    interp.0.borrow_mut().def_module::<Abbrev>(spec);
    
    
    
    interp.def_rb_source_file(
        b"abbrev.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/abbrev.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct Abbrev;


