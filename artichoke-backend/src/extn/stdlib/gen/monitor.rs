use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "MonitorMixin", None)?;
    interp.0.borrow_mut().def_module::<MonitorMixin>(spec);
    
    
    
    let spec = crate::class::Spec::new("Monitor", None, None)?;
    interp.0.borrow_mut().def_class::<Monitor>(spec);
    
    
    
    interp.def_rb_source_file(
        b"monitor.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/monitor.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct MonitorMixin;


#[derive(Debug)]
pub struct Monitor;


