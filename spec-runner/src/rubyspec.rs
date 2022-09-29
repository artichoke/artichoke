//! Embedded copy of ruby/spec suites.

use artichoke::prelude::*;
use rust_embed::RustEmbed;

/// Load ruby/spec sources into the Artichoke virtual file system.
///
/// # Errors
///
/// If an exception is raised on the Artichoke interpreter, it is returned.
pub fn init(interp: &mut Artichoke) -> Result<(), Error> {
    for source in Specs::iter() {
        if let Some(content) = Specs::get(&source) {
            interp.def_rb_source_file(&*source, content.data)?;
        }
    }
    Ok(())
}

/// ruby/spec source code.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, RustEmbed)]
#[folder = "vendor/spec"]
pub struct Specs;
