use crate::load::LoadSources;
use crate::ArtichokeError;
use crate::Mrb;

pub fn init(interp: &Mrb) -> Result<(), ArtichokeError> {
    interp.def_rb_source_file("json.rb", include_str!("json.rb"))?;
    interp.def_rb_source_file("json/common.rb", include_str!("json/common.rb"))?;
    interp.def_rb_source_file(
        "json/generic_object.rb",
        include_str!("json/generic_object.rb"),
    )?;
    interp.def_rb_source_file("json/version.rb", include_str!("json/version.rb"))?;
    interp.def_rb_source_file("json/pure.rb", include_str!("json/pure.rb"))?;
    interp.def_rb_source_file(
        "json/pure/generator.rb",
        include_str!("json/pure/generator.rb"),
    )?;
    interp.def_rb_source_file("json/pure/parser.rb", include_str!("json/pure/parser.rb"))?;
    Ok(())
}
