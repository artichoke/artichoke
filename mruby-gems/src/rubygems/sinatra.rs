use mruby::interpreter::Mrb;
use mruby::load::MrbLoadSources;
use mruby::MrbError;
use std::borrow::Cow;
use std::convert::AsRef;

use crate::Gem;

/// Load the [`Sinatra`] gem into an interpreter.
pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    Sinatra::init(interp)
}

/// Gem
#[derive(RustEmbed)]
// TODO: resolve path relative to CARGO_MANIFEST_DIR
// https://github.com/pyros2097/rust-embed/pull/59
#[folder = "mruby-gems/vendor/ruby/2.6.0/gems/sinatra-2.0.5/lib"]
struct Sinatra;

impl Sinatra {
    fn contents<T: AsRef<str>>(path: T) -> Result<Vec<u8>, MrbError> {
        let path = path.as_ref();
        let contents = Self::get(path)
            .map(Cow::into_owned)
            .ok_or_else(|| MrbError::SourceNotFound(path.to_owned()))?;
        if path == "sinatra/base.rb" {
            let mut string = String::from_utf8(contents)
                .map_err(|_| MrbError::SourceNotFound(path.to_owned()))?;
            string = string.replace("defined?(RUBY_IGNORE_CALLERS)", "false");
            string = string.replace("defined? Encoding", "false");
            string = string.replace("defined?(RUBY_ENGINE)", "false");
            string = string.replace(
                r#"class_eval("def #{name}() #{content}; end")"#,
                r#"eval("def #{name}() #{content}; end")"#,
            );
            string = string.replace(
                r"map!    { |line| line.split(/:(?=\d|in )/, 3)[0,keep] }.",
                r"map!    { |line| line.split(':', 3)[0,keep] }",
            );
            string = string.replace(
                "reject { |file, *_| CALLERS_TO_IGNORE.any? { |pattern| file =~ pattern } }",
                "# reject { |file, *_| CALLERS_TO_IGNORE.any? { |pattern| file =~ pattern } }",
            );
            string = string.replace("(not_set = true)", "NOT_SET");
            string = string.replace(
                "raise ArgumentError if block and !not_set",
                "not_set = value.not_set?; raise ArgumentError if block and !not_set",
            );
            string = string.replace("File.fnmatch(pattern, t)", "false");
            string = string.replace("builder.use ShowExceptions", "# builder.use ShowExceptions");
            string = string.replace(
                "dump_errors! boom if settings.dump_errors?",
                "# dump_errors! boom if settings.dump_errors?",
            );
            string = string.replace("&& File.expand_path(File.dirname(app_file))", "");
            string = string.replace("root && File.join(root, 'public')", "root + '/public'");
            string = string.replace("public_folder && File.exist?(public_folder)", "false");
            string = string.replace(
                "rescue\n      @env['sinatra.error.params'] = @params\n      raise",
                "rescue => e; @env['sinatra.error.params'] = @params; raise e",
            );
            Ok(string.into_bytes())
        } else {
            Ok(contents)
        }
    }
}

impl Gem for Sinatra {
    fn init(interp: &Mrb) -> Result<(), MrbError> {
        for source in Self::iter() {
            let contents = Self::contents(&source)?;
            interp.def_rb_source_file(source, contents)?;
        }
        Ok(())
    }
}
