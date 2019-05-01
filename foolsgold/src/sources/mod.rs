use std::borrow::Cow;
use std::convert::AsRef;

pub mod foolsgold;
pub mod rackup;

// TODO: resolve path relative to CARGO_MANIFEST_DIR
// https://github.com/pyros2097/rust-embed/pull/59
#[derive(RustEmbed)]
#[folder = "foolsgold/ruby/lib"]
pub struct Source;

impl Source {
    fn contents<T: AsRef<str>>(path: T) -> Vec<u8> {
        let path = path.as_ref();
        Self::get(path).map(Cow::into_owned).expect(path)
    }
}
