use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    interp.def_rb_source_file(
        b"English.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/English.rb"))[..]
    )?;
    
    Ok(())
}

