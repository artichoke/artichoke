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
        assets.insert(
            "/".to_owned(),
            Self::get("index.html")
                .map(Cow::into_owned)
                .ok_or(Error::FailedLaunch("missing index.html".to_owned()))?,
        );
        assets.insert(
            "/img/pyrite.jpg".to_owned(),
            Self::get("pyrite.jpg")
                .map(Cow::into_owned)
                .ok_or(Error::FailedLaunch("missing pyrite.jpg".to_owned()))?,
        );
        assets.insert(
            "/img/resf.png".to_owned(),
            Self::get("resf.png")
                .map(Cow::into_owned)
                .ok_or(Error::FailedLaunch("missing resf.png".to_owned()))?,
        );
        Ok(assets)
    }
}
