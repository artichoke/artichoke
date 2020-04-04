use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::class::Spec::new("ScanError", None, None)?;
    interp.0.borrow_mut().def_class::<ScanError>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "URI", None)?;
    interp.0.borrow_mut().def_module::<URI>(spec);
    
    
    
    let spec = crate::class::Spec::new("Addrinfo", None, None)?;
    interp.0.borrow_mut().def_class::<Addrinfo>(spec);
    
    
    
    let spec = crate::class::Spec::new("DateTime", None, None)?;
    interp.0.borrow_mut().def_class::<DateTime>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "RbConfig", None)?;
    interp.0.borrow_mut().def_module::<RbConfig>(spec);
    
    
    
    let spec = crate::class::Spec::new("Socket", None, None)?;
    interp.0.borrow_mut().def_class::<Socket>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Singleton", None)?;
    interp.0.borrow_mut().def_module::<Singleton>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Digest", None)?;
    interp.0.borrow_mut().def_module::<Digest>(spec);
    
    
    
    let spec = crate::class::Spec::new("BasicSocket", None, None)?;
    interp.0.borrow_mut().def_class::<BasicSocket>(spec);
    
    
    
    let spec = crate::class::Spec::new("SocketError", None, None)?;
    interp.0.borrow_mut().def_class::<SocketError>(spec);
    
    
    
    let spec = crate::class::Spec::new("TimeoutError", None, None)?;
    interp.0.borrow_mut().def_class::<TimeoutError>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Timeout", None)?;
    interp.0.borrow_mut().def_module::<Timeout>(spec);
    
    
    
    // Skipping constant CROSS_COMPILING with class NilClass
    
    
    
    let spec = crate::class::Spec::new("ERB", None, None)?;
    interp.0.borrow_mut().def_class::<ERB>(spec);
    
    
    
    let spec = crate::class::Spec::new("IPAddr", None, None)?;
    interp.0.borrow_mut().def_class::<IPAddr>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "WEBrick", None)?;
    interp.0.borrow_mut().def_module::<WEBrick>(spec);
    
    
    
    let spec = crate::class::Spec::new("Tempfile", None, None)?;
    interp.0.borrow_mut().def_class::<Tempfile>(spec);
    
    
    
    let spec = crate::class::Spec::new("StringScanner", None, None)?;
    interp.0.borrow_mut().def_class::<StringScanner>(spec);
    
    
    
    let spec = crate::class::Spec::new("Delegator", None, None)?;
    interp.0.borrow_mut().def_class::<Delegator>(spec);
    
    
    
    let spec = crate::class::Spec::new("UDPSocket", None, None)?;
    interp.0.borrow_mut().def_class::<UDPSocket>(spec);
    
    
    
    let spec = crate::class::Spec::new("IPSocket", None, None)?;
    interp.0.borrow_mut().def_class::<IPSocket>(spec);
    
    
    
    let spec = crate::class::Spec::new("TCPServer", None, None)?;
    interp.0.borrow_mut().def_class::<TCPServer>(spec);
    
    
    
    let spec = crate::class::Spec::new("TCPSocket", None, None)?;
    interp.0.borrow_mut().def_class::<TCPSocket>(spec);
    
    
    
    let spec = crate::class::Spec::new("UNIXServer", None, None)?;
    interp.0.borrow_mut().def_class::<UNIXServer>(spec);
    
    
    
    let spec = crate::class::Spec::new("UNIXSocket", None, None)?;
    interp.0.borrow_mut().def_class::<UNIXSocket>(spec);
    
    
    
    let spec = crate::class::Spec::new("SimpleDelegator", None, None)?;
    interp.0.borrow_mut().def_class::<SimpleDelegator>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Etc", None)?;
    interp.0.borrow_mut().def_module::<Etc>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "FileUtils", None)?;
    interp.0.borrow_mut().def_module::<FileUtils>(spec);
    
    
    
    let spec = crate::class::Spec::new("CGI", None, None)?;
    interp.0.borrow_mut().def_class::<CGI>(spec);
    
    
    
    let spec = crate::class::Spec::new("Date", None, None)?;
    interp.0.borrow_mut().def_class::<Date>(spec);
    
    
    
    interp.def_rb_source_file(
        b"webrick.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/accesslog.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/accesslog.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/compat.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/compat.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/config.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/config.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/cookie.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/cookie.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/htmlutils.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/htmlutils.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/httpauth.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/httpauth.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/httpauth/authenticator.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/httpauth/authenticator.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/httpauth/basicauth.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/httpauth/basicauth.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/httpauth/digestauth.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/httpauth/digestauth.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/httpauth/htdigest.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/httpauth/htdigest.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/httpauth/htgroup.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/httpauth/htgroup.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/httpauth/htpasswd.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/httpauth/htpasswd.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/httpauth/userdb.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/httpauth/userdb.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/httprequest.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/httprequest.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/httpresponse.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/httpresponse.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/httpserver.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/httpserver.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/httpservlet.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/httpservlet.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/httpservlet/abstract.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/httpservlet/abstract.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/httpservlet/cgihandler.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/httpservlet/cgihandler.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/httpservlet/erbhandler.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/httpservlet/erbhandler.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/httpservlet/filehandler.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/httpservlet/filehandler.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/httpservlet/prochandler.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/httpservlet/prochandler.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/httpstatus.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/httpstatus.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/httputils.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/httputils.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/httpversion.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/httpversion.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/log.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/log.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/server.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/server.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/utils.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/utils.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"webrick/version.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/webrick/version.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct ScanError;


#[derive(Debug)]
pub struct URI;


#[derive(Debug)]
pub struct Addrinfo;


#[derive(Debug)]
pub struct DateTime;


#[derive(Debug)]
pub struct RbConfig;


#[derive(Debug)]
pub struct Socket;


#[derive(Debug)]
pub struct Singleton;


#[derive(Debug)]
pub struct Digest;


#[derive(Debug)]
pub struct BasicSocket;


#[derive(Debug)]
pub struct SocketError;


#[derive(Debug)]
pub struct TimeoutError;


#[derive(Debug)]
pub struct Timeout;


#[derive(Debug)]
pub struct CROSS_COMPILING;


#[derive(Debug)]
pub struct ERB;


#[derive(Debug)]
pub struct IPAddr;


#[derive(Debug)]
pub struct WEBrick;


#[derive(Debug)]
pub struct Tempfile;


#[derive(Debug)]
pub struct StringScanner;


#[derive(Debug)]
pub struct Delegator;


#[derive(Debug)]
pub struct UDPSocket;


#[derive(Debug)]
pub struct IPSocket;


#[derive(Debug)]
pub struct TCPServer;


#[derive(Debug)]
pub struct TCPSocket;


#[derive(Debug)]
pub struct UNIXServer;


#[derive(Debug)]
pub struct UNIXSocket;


#[derive(Debug)]
pub struct SimpleDelegator;


#[derive(Debug)]
pub struct Etc;


#[derive(Debug)]
pub struct FileUtils;


#[derive(Debug)]
pub struct CGI;


#[derive(Debug)]
pub struct Date;


