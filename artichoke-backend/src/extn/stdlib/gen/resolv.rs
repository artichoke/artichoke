use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "SecureRandom", None)?;
    interp.0.borrow_mut().def_module::<SecureRandom>(spec);
    
    
    
    let spec = crate::class::Spec::new("UDPSocket", None, None)?;
    interp.0.borrow_mut().def_class::<UDPSocket>(spec);
    
    
    
    let spec = crate::class::Spec::new("Addrinfo", None, None)?;
    interp.0.borrow_mut().def_class::<Addrinfo>(spec);
    
    
    
    let spec = crate::class::Spec::new("TCPSocket", None, None)?;
    interp.0.borrow_mut().def_class::<TCPSocket>(spec);
    
    
    
    let spec = crate::class::Spec::new("BasicSocket", None, None)?;
    interp.0.borrow_mut().def_class::<BasicSocket>(spec);
    
    
    
    let spec = crate::class::Spec::new("SocketError", None, None)?;
    interp.0.borrow_mut().def_class::<SocketError>(spec);
    
    
    
    let spec = crate::class::Spec::new("TimeoutError", None, None)?;
    interp.0.borrow_mut().def_class::<TimeoutError>(spec);
    
    
    
    let spec = crate::class::Spec::new("Resolv", None, None)?;
    interp.0.borrow_mut().def_class::<Resolv>(spec);
    
    
    
    let spec = crate::class::Spec::new("IPSocket", None, None)?;
    interp.0.borrow_mut().def_class::<IPSocket>(spec);
    
    
    
    let spec = crate::class::Spec::new("TCPServer", None, None)?;
    interp.0.borrow_mut().def_class::<TCPServer>(spec);
    
    
    
    let spec = crate::class::Spec::new("UNIXSocket", None, None)?;
    interp.0.borrow_mut().def_class::<UNIXSocket>(spec);
    
    
    
    let spec = crate::class::Spec::new("UNIXServer", None, None)?;
    interp.0.borrow_mut().def_class::<UNIXServer>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Timeout", None)?;
    interp.0.borrow_mut().def_module::<Timeout>(spec);
    
    
    
    let spec = crate::class::Spec::new("Socket", None, None)?;
    interp.0.borrow_mut().def_class::<Socket>(spec);
    
    
    
    interp.def_rb_source_file(
        b"resolv.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/resolv.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct SecureRandom;


#[derive(Debug)]
pub struct UDPSocket;


#[derive(Debug)]
pub struct Addrinfo;


#[derive(Debug)]
pub struct TCPSocket;


#[derive(Debug)]
pub struct BasicSocket;


#[derive(Debug)]
pub struct SocketError;


#[derive(Debug)]
pub struct TimeoutError;


#[derive(Debug)]
pub struct Resolv;


#[derive(Debug)]
pub struct IPSocket;


#[derive(Debug)]
pub struct TCPServer;


#[derive(Debug)]
pub struct UNIXSocket;


#[derive(Debug)]
pub struct UNIXServer;


#[derive(Debug)]
pub struct Timeout;


#[derive(Debug)]
pub struct Socket;


