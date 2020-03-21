use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "Forwardable", None)?;
    interp.0.borrow_mut().def_module::<Forwardable>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "SingleForwardable", None)?;
    interp.0.borrow_mut().def_module::<SingleForwardable>(spec);
    
    
    
    interp.def_rb_source_file(
        b"forwardable.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/forwardable.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"forwardable/impl.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/forwardable/impl.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct Forwardable;


#[derive(Debug)]
pub struct SingleForwardable;


