use artichoke_backend::load::MrbLoadSources;
use artichoke_backend::Mrb;
use artichoke_backend::MrbError;
use std::borrow::Cow;
use std::convert::AsRef;

use crate::Gem;

/// Load the [`Rack`] gem into an interpreter.
pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    Rack::init(interp)
}

/// Gem
#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/vendor/ruby/2.6.0/gems/rack-2.0.7/lib"]
struct Rack;

impl Rack {
    fn contents<T: AsRef<str>>(path: T) -> Result<Vec<u8>, MrbError> {
        let path = path.as_ref();
        let contents = Self::get(path)
            .map(Cow::into_owned)
            .ok_or_else(|| MrbError::SourceNotFound(path.to_owned()))?;
        // patches
        if path == "rack/builder.rb" {
            let mut string = String::from_utf8(contents)
                .map_err(|_| MrbError::SourceNotFound(path.to_owned()))?;
            string = string.replace("TOPLEVEL_BINDING", "nil");
            Ok(string.into_bytes())
        } else if path == "rack/request.rb" {
            let mut string = String::from_utf8(contents)
                .map_err(|_| MrbError::SourceNotFound(path.to_owned()))?;
            string = string.replace("EOFError", "# EOFError");
            Ok(string.into_bytes())
        } else if path == "rack/utils.rb" {
            let mut string = String::from_utf8(contents)
                .map_err(|_| MrbError::SourceNotFound(path.to_owned()))?;
            string = string.replace("defined?(Process::CLOCK_MONOTONIC)", "false");
            string = string.replace("::File::SEPARATOR, ::File::ALT_SEPARATOR", "'/', nil");
            Ok(string.into_bytes())
        } else {
            Ok(contents)
        }
    }
}

impl Gem for Rack {
    fn init(interp: &Mrb) -> Result<(), MrbError> {
        for source in Self::iter() {
            let contents = Self::contents(&source)?;
            interp.def_rb_source_file(source, contents)?;
        }
        Ok(())
    }
}
