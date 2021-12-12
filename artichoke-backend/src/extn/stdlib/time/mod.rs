use crate::extn::prelude::*;

static TIME_RUBY_SOURCE: &[u8] = include_bytes!("vendor/time.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    // time package does not define any additional types; it provides extension
    // methods on the `Time` core class.
    interp.def_rb_source_file("time.rb", TIME_RUBY_SOURCE)?;

    Ok(())
}
