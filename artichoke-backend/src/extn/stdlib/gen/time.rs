use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::class::Spec::new("Date", None, None)?;
    interp.0.borrow_mut().def_class::<Date>(spec);
    
    
    
    let spec = crate::class::Spec::new("DateTime", None, None)?;
    interp.0.borrow_mut().def_class::<DateTime>(spec);
    
    
    
    interp.def_rb_source_file(
        b"time.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/time.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct Date;


#[derive(Debug)]
pub struct DateTime;


