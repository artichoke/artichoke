use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::class::Spec::new("OptParse", None, None)?;
    interp.0.borrow_mut().def_class::<OptParse>(spec);
    
    
    
    let spec = crate::class::Spec::new("OptionParser", None, None)?;
    interp.0.borrow_mut().def_class::<OptionParser>(spec);
    
    
    
    interp.def_rb_source_file(
        b"optparse.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/optparse.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct OptParse;


#[derive(Debug)]
pub struct OptionParser;


