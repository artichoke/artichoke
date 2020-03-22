use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    interp.def_rb_source_file(b"time.rb", &include_bytes!("time.rb")[..])?;
    Ok(())
}
