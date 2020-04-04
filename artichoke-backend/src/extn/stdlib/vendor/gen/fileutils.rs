use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "RbConfig", None)?;
    interp.0.borrow_mut().def_module::<RbConfig>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Etc", None)?;
    interp.0.borrow_mut().def_module::<Etc>(spec);
    
    
    
    // Skipping constant CROSS_COMPILING with class NilClass
    
    
    
    let spec = crate::module::Spec::new(interp, "FileUtils", None)?;
    interp.0.borrow_mut().def_module::<FileUtils>(spec);
    
    
    
    interp.def_rb_source_file(
        b"fileutils.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/fileutils.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"fileutils/version.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/fileutils/version.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct RbConfig;


#[derive(Debug)]
pub struct Etc;


#[derive(Debug)]
pub struct CROSS_COMPILING;


#[derive(Debug)]
pub struct FileUtils;


