use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = class::Spec::new("Delegator", None, None)?;
    interp.def_class::<Delegator>(spec)?;
    let spec = class::Spec::new("SimpleDelegator", None, None)?;
    interp.def_class::<SimpleDelegator>(spec)?;
    interp.def_rb_source_file("delegate.rb", &include_bytes!("vendor/delegate.rb")[..])?;
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Delegator;

#[derive(Debug, Clone, Copy)]
pub struct SimpleDelegator;
