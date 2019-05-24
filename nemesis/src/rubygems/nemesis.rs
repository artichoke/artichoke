use mruby::interpreter::Mrb;
use mruby::load::MrbLoadSources;
use mruby::MrbError;
use mruby_gems::Gem;
use std::borrow::Cow;
use std::convert::AsRef;

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    Nemesis::init(interp)
}

#[derive(RustEmbed)]
// TODO: resolve path relative to CARGO_MANIFEST_DIR
// https://github.com/pyros2097/rust-embed/pull/59
#[folder = "nemesis/ruby/lib"]
struct Nemesis;

impl Nemesis {
    fn contents<T: AsRef<str>>(path: T) -> Result<Vec<u8>, MrbError> {
        let path = path.as_ref();
        Self::get(path)
            .map(Cow::into_owned)
            .ok_or_else(|| MrbError::SourceNotFound(path.to_owned()))
    }
}

impl Gem for Nemesis {
    fn init(interp: &Mrb) -> Result<(), MrbError> {
        for source in Self::iter() {
            let contents = Self::contents(&source)?;
            interp.def_rb_source_file(source, contents)?;
        }
        Ok(())
    }
}
