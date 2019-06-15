use std::borrow::Cow;

use crate::interpreter::Mrb;
use crate::load::MrbLoadSources;
use crate::MrbError;

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    interp.def_rb_source_file("mspec.rb", include_str!("mspec.rb"))?;
    for source in MSpec::iter() {
        let content = MSpec::get(&source).map(Cow::into_owned).unwrap();
        interp.def_rb_source_file(source, content)?;
    }
    Ok(())
}

#[derive(RustEmbed)]
#[folder = "mruby/src/extn/test/mspec/"]
struct MSpec;
