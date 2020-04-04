use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::class::Spec::new("GetoptLong", None, None)?;
    interp.0.borrow_mut().def_class::<GetoptLong>(spec);
    
    
    
    interp.def_rb_source_file(
        b"getoptlong.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/getoptlong.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct GetoptLong;


