use mruby::load::MrbLoadSources;
use mruby::Mrb;
use mruby::MrbError;
use std::borrow::Cow;
use std::convert::AsRef;

use crate::Gem;

/// Load the [`Tilt`] gem into an interpreter.
pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    Tilt::init(interp)
}

/// Gem
#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/vendor/ruby/2.6.0/gems/tilt-2.0.9/lib"]
struct Tilt;

impl Tilt {
    fn contents<T: AsRef<str>>(path: T) -> Result<Vec<u8>, MrbError> {
        let path = path.as_ref();
        Self::get(path)
            .map(Cow::into_owned)
            .ok_or_else(|| MrbError::SourceNotFound(path.to_owned()))
    }
}

impl Gem for Tilt {
    fn init(interp: &Mrb) -> Result<(), MrbError> {
        for source in Self::iter() {
            let contents = Self::contents(&source)?;
            interp.def_rb_source_file(source, contents)?;
        }
        Ok(())
    }
}
