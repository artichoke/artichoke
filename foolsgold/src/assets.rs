use nemesis::Error;
use std::borrow::Cow;
use std::collections::HashMap;

// TODO: resolve path relative to CARGO_MANIFEST_DIR
// https://github.com/pyros2097/rust-embed/pull/59
#[derive(RustEmbed)]
#[folder = "foolsgold/static/"]
pub struct Assets;

impl Assets {
    pub fn all() -> Result<HashMap<String, Vec<u8>>, Error> {
        let mut assets = HashMap::default();
        for source in Self::iter() {
            let content = Self::get(&source)
                .map(Cow::into_owned)
                .ok_or_else(|| Error::FailedLaunch(format!("missing static asset {}", source)))?;
            if source == "index.html" {
                let route = "/".to_owned();
                assets.insert(route, content.clone());
            }
            let route = format!("/{}", source);
            assets.insert(route, content);
        }
        Ok(assets)
    }
}
