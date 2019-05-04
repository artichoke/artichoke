#[macro_use]
extern crate rust_embed;

use mruby::file::MrbFile;
use mruby::interpreter::{Mrb, MrbApi};
use std::borrow::Cow;
use std::convert::AsRef;

// TODO: resolve path relative to CARGO_MANIFEST_DIR
// https://github.com/pyros2097/rust-embed/pull/59
#[derive(RustEmbed)]
#[folder = "mruby-rack/vendor/rack-2.0.7"]
pub struct Source;

impl Source {
    fn contents<T: AsRef<str>>(path: T) -> Vec<u8> {
        let path = path.as_ref();
        Self::get(path).map(Cow::into_owned).expect(path)
    }
}

/// [`Builder`] is an empty struct that implements `MrbFile`. Requiring
/// [`Builder`] on an [`Mrb`] exposes the Ruby class
/// [`Rack::Builder`](https://github.com/rack/rack/blob/2.0.7/lib/rack/builder.rb).
///
/// `Rack::Builder` can generate a Rack-compatible app from a `config.ru`
/// rackup file.
pub struct Builder;

impl MrbFile for Builder {
    fn require(interp: Mrb) {
        let builder = Source::contents("lib/rack/builder.rb");
        interp.eval(builder).expect("rack/builder source");
    }
}
