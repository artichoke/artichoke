use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::class::Spec::new("Set", None, None)?;
    interp.0.borrow_mut().def_class::<Set>(spec);
    
    
    
    let spec = crate::class::Spec::new("SortedSet", None, None)?;
    interp.0.borrow_mut().def_class::<SortedSet>(spec);
    
    
    
    interp.def_rb_source_file(
        b"set.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/set.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct Set;


#[derive(Debug)]
pub struct SortedSet;


