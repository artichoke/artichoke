use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::class::Spec::new("Monitor", None, None)?;
    interp.0.borrow_mut().def_class::<Monitor>(spec);
    
    
    
    let spec = crate::class::Spec::new("Logger", None, None)?;
    interp.0.borrow_mut().def_class::<Logger>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "MonitorMixin", None)?;
    interp.0.borrow_mut().def_module::<MonitorMixin>(spec);
    
    
    
    interp.def_rb_source_file(
        b"logger.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/logger.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct Monitor;


#[derive(Debug)]
pub struct Logger;


#[derive(Debug)]
pub struct MonitorMixin;


