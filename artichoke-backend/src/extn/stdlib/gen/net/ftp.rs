use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "Timeout", None)?;
    interp.0.borrow_mut().def_module::<Timeout>(spec);
    
    
    
    let spec = crate::class::Spec::new("TCPServer", None, None)?;
    interp.0.borrow_mut().def_class::<TCPServer>(spec);
    
    
    
    let spec = crate::class::Spec::new("UDPSocket", None, None)?;
    interp.0.borrow_mut().def_class::<UDPSocket>(spec);
    
    
    
    let spec = crate::class::Spec::new("TCPSocket", None, None)?;
    interp.0.borrow_mut().def_class::<TCPSocket>(spec);
    
    
    
    let spec = crate::class::Spec::new("Addrinfo", None, None)?;
    interp.0.borrow_mut().def_class::<Addrinfo>(spec);
    
    
    
    let spec = crate::class::Spec::new("TimeoutError", None, None)?;
    interp.0.borrow_mut().def_class::<TimeoutError>(spec);
    
    
    
    let spec = crate::class::Spec::new("Date", None, None)?;
    interp.0.borrow_mut().def_class::<Date>(spec);
    
    
    
    let spec = crate::class::Spec::new("Monitor", None, None)?;
    interp.0.borrow_mut().def_class::<Monitor>(spec);
    
    
    
    let spec = crate::class::Spec::new("StringIO", None, None)?;
    interp.0.borrow_mut().def_class::<StringIO>(spec);
    
    
    
    let spec = crate::class::Spec::new("DateTime", None, None)?;
    interp.0.borrow_mut().def_class::<DateTime>(spec);
    
    
    
    let spec = crate::class::Spec::new("BasicSocket", None, None)?;
    interp.0.borrow_mut().def_class::<BasicSocket>(spec);
    
    
    
    let spec = crate::class::Spec::new("SocketError", None, None)?;
    interp.0.borrow_mut().def_class::<SocketError>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Net", None)?;
    interp.0.borrow_mut().def_module::<Net>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "MonitorMixin", None)?;
    interp.0.borrow_mut().def_module::<MonitorMixin>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "OpenSSL", None)?;
    interp.0.borrow_mut().def_module::<OpenSSL>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Digest", None)?;
    interp.0.borrow_mut().def_module::<Digest>(spec);
    
    
    
    let spec = crate::class::Spec::new("IPAddr", None, None)?;
    interp.0.borrow_mut().def_class::<IPAddr>(spec);
    
    
    
    let spec = crate::class::Spec::new("IPSocket", None, None)?;
    interp.0.borrow_mut().def_class::<IPSocket>(spec);
    
    
    
    let spec = crate::class::Spec::new("Socket", None, None)?;
    interp.0.borrow_mut().def_class::<Socket>(spec);
    
    
    
    let spec = crate::class::Spec::new("UNIXServer", None, None)?;
    interp.0.borrow_mut().def_class::<UNIXServer>(spec);
    
    
    
    let spec = crate::class::Spec::new("UNIXSocket", None, None)?;
    interp.0.borrow_mut().def_class::<UNIXSocket>(spec);
    
    
    
    interp.def_rb_source_file(
        b"net/ftp.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/net/ftp.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct Timeout;


#[derive(Debug)]
pub struct TCPServer;


#[derive(Debug)]
pub struct UDPSocket;


#[derive(Debug)]
pub struct TCPSocket;


#[derive(Debug)]
pub struct Addrinfo;


#[derive(Debug)]
pub struct TimeoutError;


#[derive(Debug)]
pub struct Date;


#[derive(Debug)]
pub struct Monitor;


#[derive(Debug)]
pub struct StringIO;


#[derive(Debug)]
pub struct DateTime;


#[derive(Debug)]
pub struct BasicSocket;


#[derive(Debug)]
pub struct SocketError;


#[derive(Debug)]
pub struct Net;


#[derive(Debug)]
pub struct MonitorMixin;


#[derive(Debug)]
pub struct OpenSSL;


#[derive(Debug)]
pub struct Digest;


#[derive(Debug)]
pub struct IPAddr;


#[derive(Debug)]
pub struct IPSocket;


#[derive(Debug)]
pub struct Socket;


#[derive(Debug)]
pub struct UNIXServer;


#[derive(Debug)]
pub struct UNIXSocket;


