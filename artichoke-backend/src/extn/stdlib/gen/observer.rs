use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "Observable", None)?;
    interp.0.borrow_mut().def_module::<Observable>(spec);
    
    
    
    interp.def_rb_source_file(
        b"observer.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/observer.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct Observable;


