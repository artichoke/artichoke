use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "Mutex_m", None)?;
    interp.0.borrow_mut().def_module::<Mutex_m>(spec);
    
    
    
    interp.def_rb_source_file(
        b"mutex_m.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/mutex_m.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct Mutex_m;


