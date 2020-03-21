use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "NKF", None)?;
    interp.0.borrow_mut().def_module::<NKF>(spec);
    
    
    
    Ok(())
}

#[derive(Debug)]
pub struct NKF;


