use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = class::Spec::new("Set", None, None)?;
    interp.def_class::<Set>(spec)?;
    let spec = class::Spec::new("SortedSet", None, None)?;
    interp.def_class::<SortedSet>(spec)?;
    interp.def_rb_source_file("set.rb", &include_bytes!("vendor/set.rb")[..])?;
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Set;

#[derive(Debug, Clone, Copy)]
#[allow(clippy::module_name_repetitions)]
pub struct SortedSet;
