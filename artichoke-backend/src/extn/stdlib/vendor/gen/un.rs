use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::class::Spec::new("OptParse", None, None)?;
    interp.0.borrow_mut().def_class::<OptParse>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Etc", None)?;
    interp.0.borrow_mut().def_module::<Etc>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "RbConfig", None)?;
    interp.0.borrow_mut().def_module::<RbConfig>(spec);
    
    
    
    // Skipping constant CROSS_COMPILING with class NilClass
    
    
    
    let spec = crate::module::Spec::new(interp, "FileUtils", None)?;
    interp.0.borrow_mut().def_module::<FileUtils>(spec);
    
    
    
    let spec = crate::class::Spec::new("OptionParser", None, None)?;
    interp.0.borrow_mut().def_class::<OptionParser>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "UN", None)?;
    interp.0.borrow_mut().def_module::<UN>(spec);
    
    
    
    interp.def_rb_source_file(
        b"un.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/un.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct OptParse;


#[derive(Debug)]
pub struct Etc;


#[derive(Debug)]
pub struct RbConfig;


#[derive(Debug)]
pub struct CROSS_COMPILING;


#[derive(Debug)]
pub struct FileUtils;


#[derive(Debug)]
pub struct OptionParser;


#[derive(Debug)]
pub struct UN;


