use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = class::Spec::new("Delegator", None, None)?;
    interp.state_mut().def_class::<Delegator>(spec);
    let spec = class::Spec::new("SimpleDelegator", None, None)?;
    interp.state_mut().def_class::<SimpleDelegator>(spec);
    interp.def_rb_source_file(b"delegate.rb", &include_bytes!("delegate.rb")[..])?;
    Ok(())
}

pub struct Delegator;
pub struct SimpleDelegator;
