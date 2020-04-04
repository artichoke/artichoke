use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::class::Spec::new("PrettyPrint", None, None)?;
    interp.0.borrow_mut().def_class::<PrettyPrint>(spec);
    
    
    
    interp.def_rb_source_file(
        b"prettyprint.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/prettyprint.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct PrettyPrint;


