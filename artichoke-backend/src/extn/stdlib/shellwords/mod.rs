use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = module::Spec::new(interp, "Shellwords", None)?;
    interp.0.borrow_mut().def_module::<Shellwords>(spec);
    interp.def_rb_source_file(
        b"shellwords.rb",
        &include_bytes!("vendor/shellwords.rb")[..],
    )?;
    Ok(())
}

#[derive(Debug)]
pub struct Shellwords;
