use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::class::Spec::new("CGI", None, None)?;
    interp.0.borrow_mut().def_class::<CGI>(spec);
    
    
    
    interp.def_rb_source_file(
        b"cgi.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/cgi.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"cgi/cookie.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/cgi/cookie.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"cgi/core.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/cgi/core.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"cgi/util.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/cgi/util.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct CGI;


