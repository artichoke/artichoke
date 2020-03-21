use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "Open3", None)?;
    interp.0.borrow_mut().def_module::<Open3>(spec);
    
    
    
    interp.def_rb_source_file(
        b"open3.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/open3.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct Open3;


