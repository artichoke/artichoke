use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "Singleton", None)?;
    interp.0.borrow_mut().def_module::<Singleton>(spec);
    
    
    
    interp.def_rb_source_file(
        b"singleton.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/singleton.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct Singleton;


