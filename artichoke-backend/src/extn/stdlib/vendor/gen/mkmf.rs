use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    // Skipping constant CROSS_COMPILING with class NilClass
    
    
    
    let spec = crate::module::Spec::new(interp, "Etc", None)?;
    interp.0.borrow_mut().def_module::<Etc>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "MakeMakefile", None)?;
    interp.0.borrow_mut().def_module::<MakeMakefile>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "RbConfig", None)?;
    interp.0.borrow_mut().def_module::<RbConfig>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Shellwords", None)?;
    interp.0.borrow_mut().def_module::<Shellwords>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "FileUtils", None)?;
    interp.0.borrow_mut().def_module::<FileUtils>(spec);
    
    
    
    // Skipping constant EXPORT_PREFIX with class FalseClass
    
    
    
    // Skipping constant COMMON_LIBS with class Array
    
    
    
    // Skipping constant INSTALL_DIRS with class Array
    
    
    
    // Skipping constant COMPILE_CXX with class String
    
    
    
    // Skipping constant ASSEMBLE_CXX with class String
    
    
    
    // Skipping constant COMPILE_C with class String
    
    
    
    // Skipping constant STRING_OR_FAILED_FORMAT with class String
    
    
    
    // Skipping constant CONFTEST with class String
    
    
    
    // Skipping constant CONFTEST_C with class String
    
    
    
    // Skipping constant OUTFLAG with class String
    
    
    
    // Skipping constant RULE_SUBST with class NilClass
    
    
    
    // Skipping constant COUTFLAG with class String
    
    
    
    // Skipping constant CSRCFLAG with class String
    
    
    
    // Skipping constant CPPOUTFILE with class String
    
    
    
    // Skipping constant LINK_SO with class String
    
    
    
    // Skipping constant FailedMessage with class String
    
    
    
    // Skipping constant ASSEMBLE_C with class String
    
    
    
    // Skipping constant COMMON_HEADERS with class String
    
    
    
    // Skipping constant MAIN_DOES_NOTHING with class String
    
    
    
    // Skipping constant CONFIG with class Hash
    
    
    
    // Skipping constant UNIVERSAL_INTS with class Array
    
    
    
    // Skipping constant CLEANINGS with class String
    
    
    
    // Skipping constant ORIG_LIBPATH with class NilClass
    
    
    
    // Skipping constant C_EXT with class Array
    
    
    
    // Skipping constant CXX_EXT with class Array
    
    
    
    // Skipping constant COMPILE_RULES with class Array
    
    
    
    // Skipping constant SRC_EXT with class Array
    
    
    
    // Skipping constant HDR_EXT with class Array
    
    
    
    // Skipping constant TRY_LINK with class String
    
    
    
    let spec = crate::module::Spec::new(interp, "Logging", None)?;
    interp.0.borrow_mut().def_module::<Logging>(spec);
    
    
    
    // Skipping constant LIBPATHFLAG with class String
    
    
    
    // Skipping constant RPATHFLAG with class String
    
    
    
    // Skipping constant LIBARG with class String
    
    
    
    interp.def_rb_source_file(
        b"mkmf.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/mkmf.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct CROSS_COMPILING;


#[derive(Debug)]
pub struct Etc;


#[derive(Debug)]
pub struct MakeMakefile;


#[derive(Debug)]
pub struct RbConfig;


#[derive(Debug)]
pub struct Shellwords;


#[derive(Debug)]
pub struct FileUtils;


#[derive(Debug)]
pub struct EXPORT_PREFIX;


#[derive(Debug)]
pub struct COMMON_LIBS;


#[derive(Debug)]
pub struct INSTALL_DIRS;


#[derive(Debug)]
pub struct COMPILE_CXX;


#[derive(Debug)]
pub struct ASSEMBLE_CXX;


#[derive(Debug)]
pub struct COMPILE_C;


#[derive(Debug)]
pub struct STRING_OR_FAILED_FORMAT;


#[derive(Debug)]
pub struct CONFTEST;


#[derive(Debug)]
pub struct CONFTEST_C;


#[derive(Debug)]
pub struct OUTFLAG;


#[derive(Debug)]
pub struct RULE_SUBST;


#[derive(Debug)]
pub struct COUTFLAG;


#[derive(Debug)]
pub struct CSRCFLAG;


#[derive(Debug)]
pub struct CPPOUTFILE;


#[derive(Debug)]
pub struct LINK_SO;


#[derive(Debug)]
pub struct FailedMessage;


#[derive(Debug)]
pub struct ASSEMBLE_C;


#[derive(Debug)]
pub struct COMMON_HEADERS;


#[derive(Debug)]
pub struct MAIN_DOES_NOTHING;


#[derive(Debug)]
pub struct CONFIG;


#[derive(Debug)]
pub struct UNIVERSAL_INTS;


#[derive(Debug)]
pub struct CLEANINGS;


#[derive(Debug)]
pub struct ORIG_LIBPATH;


#[derive(Debug)]
pub struct C_EXT;


#[derive(Debug)]
pub struct CXX_EXT;


#[derive(Debug)]
pub struct COMPILE_RULES;


#[derive(Debug)]
pub struct SRC_EXT;


#[derive(Debug)]
pub struct HDR_EXT;


#[derive(Debug)]
pub struct TRY_LINK;


#[derive(Debug)]
pub struct Logging;


#[derive(Debug)]
pub struct LIBPATHFLAG;


#[derive(Debug)]
pub struct RPATHFLAG;


#[derive(Debug)]
pub struct LIBARG;


