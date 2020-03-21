use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "Find", None)?;
    interp.0.borrow_mut().def_module::<Find>(spec);
    
    
    
    interp.def_rb_source_file(
        b"find.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/find.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct Find;


