//! Embedded copy of ruby/spec suites.

use artichoke_backend::exception::Exception;
use artichoke_backend::{Artichoke, LoadSources};

/// Load ruby/spec sources into the Artichoke virtual filesystem.
///
/// # Errors
///
/// If an exception is raised on the Artichoke interpreter, it is returned.
pub fn init(interp: &mut Artichoke) -> Result<(), Exception> {
    for source in Specs::iter() {
        if let Some(content) = Specs::get(&source) {
            interp.def_rb_source_file(source.as_bytes(), content)?;
        }
    }
    Ok(())
}

/// ruby/spec source code.
#[derive(RustEmbed)]
#[folder = "vendor/spec"]
pub struct Specs;
