use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "Exception2MessageMapper", None)?;
    interp.0.borrow_mut().def_module::<Exception2MessageMapper>(spec);
    
    
    
    interp.def_rb_source_file(
        b"e2mmap.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/e2mmap.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct Exception2MessageMapper;


