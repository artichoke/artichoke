use crate::interpreter::Mrb;
use crate::load::MrbLoadSources;
use crate::MrbError;

pub fn patch(interp: &Mrb) -> Result<(), MrbError> {
    interp.def_rb_source_file("erb.rb", "class ERB; def initialize(*args); end; end")?;
    interp.def_rb_source_file("time.rb", "")?;
    interp.def_rb_source_file("fileutils.rb", "")?;
    interp.def_rb_source_file("tempfile.rb", "")?;
    Ok(())
}
