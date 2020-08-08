use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = module::Spec::new(interp, "Shellwords", None)?;
    interp.def_module::<Shellwords>(spec)?;
    interp.def_rb_source_file("shellwords.rb", &include_bytes!("vendor/shellwords.rb")[..])?;
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Shellwords;
