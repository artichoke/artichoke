use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = crate::module::Spec::new(interp, "Base64", None)?;
    interp.def_module::<Base64>(spec)?;
    interp.def_rb_source_file("base64.rb", &include_bytes!("vendor/base64.rb")[..])?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Base64;
