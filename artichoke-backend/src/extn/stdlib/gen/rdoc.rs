use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "RDoc", None)?;
    interp.0.borrow_mut().def_module::<RDoc>(spec);
    
    
    
    interp.def_rb_source_file(
        b"rdoc.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rdoc.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rdoc/i18n.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rdoc/i18n.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rdoc/i18n/text.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rdoc/i18n/text.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rdoc/version.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rdoc/version.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct RDoc;


