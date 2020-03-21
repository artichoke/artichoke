use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "Base64", None)?;
    interp.0.borrow_mut().def_module::<Base64>(spec);
    
    
    
    interp.def_rb_source_file(
        b"base64.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/base64.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct Base64;


