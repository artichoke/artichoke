use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "ExceptionForMatrix", None)?;
    interp.0.borrow_mut().def_module::<ExceptionForMatrix>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Exception2MessageMapper", None)?;
    interp.0.borrow_mut().def_module::<Exception2MessageMapper>(spec);
    
    
    
    let spec = crate::class::Spec::new("Matrix", None, None)?;
    interp.0.borrow_mut().def_class::<Matrix>(spec);
    
    
    
    let spec = crate::class::Spec::new("Vector", None, None)?;
    interp.0.borrow_mut().def_class::<Vector>(spec);
    
    
    
    interp.def_rb_source_file(
        b"matrix.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/matrix.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct ExceptionForMatrix;


#[derive(Debug)]
pub struct Exception2MessageMapper;


#[derive(Debug)]
pub struct Matrix;


#[derive(Debug)]
pub struct Vector;


