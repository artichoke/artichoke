use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "SingleForwardable", None)?;
    interp.0.borrow_mut().def_module::<SingleForwardable>(spec);
    
    
    
    let spec = crate::class::Spec::new("Sync", None, None)?;
    interp.0.borrow_mut().def_class::<Sync>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Sync_m", None)?;
    interp.0.borrow_mut().def_module::<Sync_m>(spec);
    
    
    
    let spec = crate::class::Spec::new("Shell", None, None)?;
    interp.0.borrow_mut().def_class::<Shell>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Exception2MessageMapper", None)?;
    interp.0.borrow_mut().def_module::<Exception2MessageMapper>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Forwardable", None)?;
    interp.0.borrow_mut().def_module::<Forwardable>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Synchronizer_m", None)?;
    interp.0.borrow_mut().def_module::<Synchronizer_m>(spec);
    
    
    
    let spec = crate::class::Spec::new("Synchronizer", None, None)?;
    interp.0.borrow_mut().def_class::<Synchronizer>(spec);
    
    
    
    interp.def_rb_source_file(
        b"shell.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/shell.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"shell/builtin-command.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/shell/builtin-command.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"shell/command-processor.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/shell/command-processor.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"shell/error.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/shell/error.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"shell/filter.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/shell/filter.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"shell/process-controller.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/shell/process-controller.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"shell/system-command.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/shell/system-command.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"shell/version.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/shell/version.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct SingleForwardable;


#[derive(Debug)]
pub struct Sync;


#[derive(Debug)]
pub struct Sync_m;


#[derive(Debug)]
pub struct Shell;


#[derive(Debug)]
pub struct Exception2MessageMapper;


#[derive(Debug)]
pub struct Forwardable;


#[derive(Debug)]
pub struct Synchronizer_m;


#[derive(Debug)]
pub struct Synchronizer;


