use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::class::Spec::new("SocketError", None, None)?;
    interp.0.borrow_mut().def_class::<SocketError>(spec);
    
    
    
    let spec = crate::class::Spec::new("BasicSocket", None, None)?;
    interp.0.borrow_mut().def_class::<BasicSocket>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Timeout", None)?;
    interp.0.borrow_mut().def_module::<Timeout>(spec);
    
    
    
    let spec = crate::class::Spec::new("TCPSocket", None, None)?;
    interp.0.borrow_mut().def_class::<TCPSocket>(spec);
    
    
    
    let spec = crate::class::Spec::new("Socket", None, None)?;
    interp.0.borrow_mut().def_class::<Socket>(spec);
    
    
    
    let spec = crate::class::Spec::new("TimeoutError", None, None)?;
    interp.0.borrow_mut().def_class::<TimeoutError>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Zlib", None)?;
    interp.0.borrow_mut().def_module::<Zlib>(spec);
    
    
    
    let spec = crate::class::Spec::new("IPSocket", None, None)?;
    interp.0.borrow_mut().def_class::<IPSocket>(spec);
    
    
    
    let spec = crate::class::Spec::new("TCPServer", None, None)?;
    interp.0.borrow_mut().def_class::<TCPServer>(spec);
    
    
    
    let spec = crate::class::Spec::new("UNIXSocket", None, None)?;
    interp.0.borrow_mut().def_class::<UNIXSocket>(spec);
    
    
    
    let spec = crate::class::Spec::new("UDPSocket", None, None)?;
    interp.0.borrow_mut().def_class::<UDPSocket>(spec);
    
    
    
    let spec = crate::class::Spec::new("UNIXServer", None, None)?;
    interp.0.borrow_mut().def_class::<UNIXServer>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "OpenSSL", None)?;
    interp.0.borrow_mut().def_module::<OpenSSL>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Net", None)?;
    interp.0.borrow_mut().def_module::<Net>(spec);
    
    
    
    let spec = crate::class::Spec::new("StringIO", None, None)?;
    interp.0.borrow_mut().def_class::<StringIO>(spec);
    
    
    
    let spec = crate::class::Spec::new("IPAddr", None, None)?;
    interp.0.borrow_mut().def_class::<IPAddr>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "URI", None)?;
    interp.0.borrow_mut().def_module::<URI>(spec);
    
    
    
    let spec = crate::class::Spec::new("Addrinfo", None, None)?;
    interp.0.borrow_mut().def_class::<Addrinfo>(spec);
    
    
    
    interp.def_rb_source_file(
        b"net/http.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/net/http.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"net/http/backward.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/net/http/backward.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"net/http/exceptions.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/net/http/exceptions.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"net/http/generic_request.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/net/http/generic_request.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"net/http/header.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/net/http/header.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"net/http/proxy_delta.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/net/http/proxy_delta.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"net/http/request.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/net/http/request.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"net/http/requests.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/net/http/requests.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"net/http/response.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/net/http/response.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"net/http/responses.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/net/http/responses.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct SocketError;


#[derive(Debug)]
pub struct BasicSocket;


#[derive(Debug)]
pub struct Timeout;


#[derive(Debug)]
pub struct TCPSocket;


#[derive(Debug)]
pub struct Socket;


#[derive(Debug)]
pub struct TimeoutError;


#[derive(Debug)]
pub struct Zlib;


#[derive(Debug)]
pub struct IPSocket;


#[derive(Debug)]
pub struct TCPServer;


#[derive(Debug)]
pub struct UNIXSocket;


#[derive(Debug)]
pub struct UDPSocket;


#[derive(Debug)]
pub struct UNIXServer;


#[derive(Debug)]
pub struct OpenSSL;


#[derive(Debug)]
pub struct Net;


#[derive(Debug)]
pub struct StringIO;


#[derive(Debug)]
pub struct IPAddr;


#[derive(Debug)]
pub struct URI;


#[derive(Debug)]
pub struct Addrinfo;


