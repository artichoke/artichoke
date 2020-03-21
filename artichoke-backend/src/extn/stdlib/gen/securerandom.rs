use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "SecureRandom", None)?;
    interp.0.borrow_mut().def_module::<SecureRandom>(spec);
    
    
    
    interp.def_rb_source_file(
        b"securerandom.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/securerandom.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct SecureRandom;


