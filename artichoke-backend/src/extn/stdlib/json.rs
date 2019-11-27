use artichoke_core::load::LoadSources;

use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp.def_rb_source_file(b"json.rb", &include_bytes!("json.rb")[..])?;
    interp.def_rb_source_file(b"json/common.rb", &include_bytes!("json/common.rb")[..])?;
    interp.def_rb_source_file(
        b"json/generic_object.rb",
        &include_bytes!("json/generic_object.rb")[..],
    )?;
    interp.def_rb_source_file(b"json/version.rb", &include_bytes!("json/version.rb")[..])?;
    interp.def_rb_source_file(b"json/pure.rb", &include_bytes!("json/pure.rb")[..])?;
    interp.def_rb_source_file(
        b"json/pure/generator.rb",
        &include_bytes!("json/pure/generator.rb")[..],
    )?;
    interp.def_rb_source_file(
        b"json/pure/parser.rb",
        &include_bytes!("json/pure/parser.rb")[..],
    )?;
    Ok(())
}
