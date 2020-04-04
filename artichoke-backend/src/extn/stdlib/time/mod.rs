use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    // time package does not define any additional types; it provides extension
    // methods on the `Time` core class.
    interp.def_rb_source_file(b"time.rb", &include_bytes!("vendor/time.rb")[..])?;
    Ok(())
}
