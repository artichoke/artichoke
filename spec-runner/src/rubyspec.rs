use artichoke_backend::exception::Exception;
use artichoke_backend::{Artichoke, LoadSources};

pub fn init(interp: &mut Artichoke) -> Result<(), Exception> {
    for source in Specs::iter() {
        if let Some(content) = Specs::get(&source) {
            interp.def_rb_source_file(source.as_bytes(), content)?;
        }
    }
    Ok(())
}

#[derive(RustEmbed)]
#[folder = "vendor/spec"]
pub struct Specs;
