use mruby::load::MrbLoadSources;
use mruby::Mrb;
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
        if path == "mustermann.rb" {
            let mut string = String::from_utf8(contents)
                .map_err(|_| MrbError::SourceNotFound(path.to_owned()))?;
            string = string.replace("defined?(Pry) or defined?(IRB)", "false");
            string = string.replace(
                "defined? ::Sinatra::Base",
                "Object.const_defined?(:Sinatra)",
            );
            Ok(string.into_bytes())
        } else if path == "mustermann/error.rb" {
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
        } else if path == "mustermann/ast/boundaries.rb" {
            let mut string = String::from_utf8(contents)
                .map_err(|_| MrbError::SourceNotFound(path.to_owned()))?;
            string = string.replace(
                "def self.set_boundaries(ast, string: nil, start: 0, stop: string.length)",
                "def self.set_boundaries(ast, string: nil, start: 0, stop: nil); stop = string&.length if stop.nil?"
            );
            Ok(string.into_bytes())
        } else if path == "mustermann/ast/compiler.rb" {
            let mut string = String::from_utf8(contents)
                .map_err(|_| MrbError::SourceNotFound(path.to_owned()))?;
            string = string.replace("private_constant", "# private_constant");
            Ok(string.into_bytes())
        } else if path == "mustermann/ast/node.rb" {
            let mut string = String::from_utf8(contents)
                .map_err(|_| MrbError::SourceNotFound(path.to_owned()))?;
            string = string.replace(
                "Object.const_get(const_name) if Object.const_defined?(const_name)",
                "Object.const_get(const_name) rescue NameError nil",
            );
            Ok(string.into_bytes())
        } else if path == "mustermann/ast/translator.rb" {
            let mut string = String::from_utf8(contents)
                .map_err(|_| MrbError::SourceNotFound(path.to_owned()))?;
            string = string.replace(
                "Class.new(self, &block).new",
                "translator = Class.new(self); translator.class_eval(&block); translator.new",
            );
            string = string.replace("private_constant", "# private_constant");
            string = string.replace(
                "def escape(char, parser: URI::DEFAULT_PARSER, escape: parser.regexp[:UNSAFE], also_escape: nil)",
                "def escape(char, parser: URI::DEFAULT_PARSER, escape: NOT_SET, also_escape: nil); escape = parser.regexp[:UNSAFE] if escape.not_set?",
            );
            Ok(string.into_bytes())
        } else if path == "mustermann/equality_map.rb" {
            let mut string = String::from_utf8(contents)
                .map_err(|_| MrbError::SourceNotFound(path.to_owned()))?;
            string = string.replace("defined?(ObjectSpace::WeakMap)", "false");
            Ok(string.into_bytes())
        } else if path == "mustermann/regexp_based.rb" {
            let mut string = String::from_utf8(contents)
                .map_err(|_| MrbError::SourceNotFound(path.to_owned()))?;
            string = string.replace("super", "super(string, **options)");
            Ok(string.into_bytes())
        } else if path == "mustermann/regular.rb" {
            let mut string = String::from_utf8(contents)
                .map_err(|_| MrbError::SourceNotFound(path.to_owned()))?;
            string = string.replace(
                r#"string = $1 if string.to_s =~ /\A\(\?\-mix\:(.*)\)\Z/ && string.inspect == "/#$1/""#,
                r#"match = /\A\(\?\-mix\:(.*)\)\Z/.match(string.to_s); string = match[1] if match && string.inspect == "/#{match[1]}/""#
            );
            Ok(string.into_bytes())
        } else if path == "mustermann/sinatra/parser.rb" {
            let mut string = String::from_utf8(contents)
                .map_err(|_| MrbError::SourceNotFound(path.to_owned()))?;
            string = string.replace("private_constant", "# private_constant");
            Ok(string.into_bytes())
        } else if path == "mustermann/sinatra/safe_renderer.rb" {
            let mut string = String::from_utf8(contents)
                .map_err(|_| MrbError::SourceNotFound(path.to_owned()))?;
            string = string.replace("private_constant", "# private_constant");
            Ok(string.into_bytes())
        } else if path == "mustermann/sinatra/try_convert.rb" {
            let mut string = String::from_utf8(contents)
                .map_err(|_| MrbError::SourceNotFound(path.to_owned()))?;
            string = string.replace("private_constant", "# private_constant");
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
