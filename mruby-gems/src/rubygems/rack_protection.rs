use mruby::load::MrbLoadSources;
use mruby::Mrb;
use mruby::MrbError;
use std::borrow::Cow;
use std::convert::AsRef;

use crate::Gem;

/// Load the [`RackProtection`] gem into an interpreter.
pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    RackProtection::init(interp)
}

/// Gem
#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/vendor/ruby/2.6.0/gems/rack-protection-2.0.5/lib"]
struct RackProtection;

impl RackProtection {
    fn contents<T: AsRef<str>>(path: T) -> Result<Vec<u8>, MrbError> {
        let path = path.as_ref();
        let contents = Self::get(path)
            .map(Cow::into_owned)
            .ok_or_else(|| MrbError::SourceNotFound(path.to_owned()))?;
        // patches
        if path == "rack/protection/base.rb" {
            let mut string = String::from_utf8(contents)
                .map_err(|_| MrbError::SourceNotFound(path.to_owned()))?;
            string = string.replace(
                "define_method(:default_options) { super().merge(options) }",
                "define_method(:default_options) { @default_options ||= DEFAULT_OPTIONS; @default_options.merge(options) }",
            );
            string = string.replace(
                "def default_options\n        DEFAULT_OPTIONS\n      end",
                "def default_options; @default_options ||= DEFAULT_OPTIONS; end",
            );
            string = string.replace("defined? SecureRandom", "true");
            Ok(string.into_bytes())
        } else {
            Ok(contents)
        }
    }
}

impl Gem for RackProtection {
    fn init(interp: &Mrb) -> Result<(), MrbError> {
        for source in Self::iter() {
            let contents = Self::contents(&source)?;
            interp.def_rb_source_file(source, contents)?;
        }
        Ok(())
    }
}
