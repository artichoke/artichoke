use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::class::Spec::new("IPSocket", None, None)?;
    interp.0.borrow_mut().def_class::<IPSocket>(spec);
    
    
    
    let spec = crate::class::Spec::new("IPAddr", None, None)?;
    interp.0.borrow_mut().def_class::<IPAddr>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "URI", None)?;
    interp.0.borrow_mut().def_module::<URI>(spec);
    
    
    
    interp.def_rb_source_file(
        b"uri.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/uri.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"uri/common.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/uri/common.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"uri/file.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/uri/file.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"uri/ftp.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/uri/ftp.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"uri/generic.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/uri/generic.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"uri/http.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/uri/http.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"uri/https.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/uri/https.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"uri/ldap.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/uri/ldap.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"uri/ldaps.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/uri/ldaps.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"uri/mailto.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/uri/mailto.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"uri/rfc2396_parser.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/uri/rfc2396_parser.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"uri/rfc3986_parser.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/uri/rfc3986_parser.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct IPSocket;


#[derive(Debug)]
pub struct IPAddr;


#[derive(Debug)]
pub struct URI;


