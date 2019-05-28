use mruby::interpreter::Mrb;
use mruby::load::MrbLoadSources;
use mruby::MrbError;
use std::borrow::Cow;
use std::convert::AsRef;

use crate::Gem;

/// Load the [`Mustermann`] gem into an interpreter.
pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    Mustermann::init(interp)
}

/// Gem
#[derive(RustEmbed)]
// TODO: resolve path relative to CARGO_MANIFEST_DIR
// https://github.com/pyros2097/rust-embed/pull/59
#[folder = "mruby-gems/vendor/ruby/2.6.0/gems/mustermann-1.0.3/lib"]
struct Mustermann;

impl Mustermann {
    fn contents<T: AsRef<str>>(path: T) -> Result<Vec<u8>, MrbError> {
        let path = path.as_ref();
        let contents = Self::get(path)
            .map(Cow::into_owned)
            .ok_or_else(|| MrbError::SourceNotFound(path.to_owned()))?;
        // patches
        if path == "mustermann/error.rb" {
            let mut string = String::from_utf8(contents)
                .map_err(|_| MrbError::SourceNotFound(path.to_owned()))?;
            string = string.replace(
                "defined?(Mustermann::Error)",
                "Mustermann.const_defined?(:Error)",
            );
            Ok(string.into_bytes())
        } else if path == "mustermann/pattern.rb" {
            let mut string = String::from_utf8(contents)
                .map_err(|_| MrbError::SourceNotFound(path.to_owned()))?;
            string = string.replace("private_constant", "# private_constant");
            Ok(string.into_bytes())
        } else if path == "mustermann.rb" {
            let mut string = String::from_utf8(contents)
                .map_err(|_| MrbError::SourceNotFound(path.to_owned()))?;
            string = string.replace("defined?(Pry) or defined?(IRB)", "false");
            string = string.replace(
                "defined? ::Sinatra::Base",
                "Object.const_defined?(:Sinatra)",
            );
            Ok(string.into_bytes())
        } else {
            Ok(contents)
        }
    }
}

impl Gem for Mustermann {
    fn init(interp: &Mrb) -> Result<(), MrbError> {
        for source in Self::iter() {
            let contents = Self::contents(&source)?;
            interp.def_rb_source_file(source, contents)?;
        }
        Ok(())
    }
}
