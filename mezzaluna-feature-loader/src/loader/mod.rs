#[cfg(feature = "rubylib")]
use std::ffi::OsStr;
use std::io;
use std::path::Path;

#[cfg(feature = "rubylib")]
mod rubylib;

#[cfg(feature = "rubylib")]
pub use rubylib::Rubylib;

use crate::loaded_features::LoadedFeatures;

#[derive(Debug)]
pub struct Loader {
    #[cfg(feature = "rubylib")]
    rubylib: Rubylib,
    #[allow(dead_code)]
    loaded_features: LoadedFeatures,
}

impl Loader {
    #[must_use]
    pub fn new() -> Option<Self> {
        #[cfg(feature = "rubylib")]
        let rubylib = Rubylib::new()?;
        let loaded_features = LoadedFeatures::new();
        Some(Self {
            #[cfg(feature = "rubylib")]
            rubylib,
            loaded_features,
        })
    }

    #[must_use]
    #[cfg(feature = "rubylib")]
    pub fn with_rubylib(rubylib: &OsStr) -> Option<Self> {
        let rubylib = Rubylib::with_rubylib(rubylib)?;
        let loaded_features = LoadedFeatures::new();
        Some(Self {
            rubylib,
            loaded_features,
        })
    }

    #[must_use]
    #[cfg(feature = "rubylib")]
    pub fn with_rubylib_and_cwd(rubylib: &OsStr, cwd: &Path) -> Option<Self> {
        #[cfg(feature = "rubylib")]
        let rubylib = Rubylib::with_rubylib_and_cwd(rubylib, cwd)?;
        let loaded_features = LoadedFeatures::new();
        Some(Self {
            rubylib,
            loaded_features,
        })
    }

    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::missing_panics_doc)]
    pub fn read<T>(&self, path: &Path) -> io::Result<Vec<u8>> {
        #[cfg(feature = "rubylib")]
        {
            use std::io::Read;

            if let Some(handle) = self.rubylib.resolve_file(path) {
                let file = handle.as_file();
                // Allocate one extra byte so the buffer doesn't need to grow before the
                // final `read` call at the end of the file.  Don't worry about `usize`
                // overflow because reading will fail regardless in that case.
                #[allow(clippy::cast_possible_truncation)]
                let initial_buffer_size = file.metadata().map(|m| m.len() as usize + 1).unwrap_or(0);
                let mut buf = Vec::with_capacity(initial_buffer_size);
                handle.as_file().read_to_end(&mut buf)?;
                return Ok(buf);
            }
        }
        let _ignore_not_implemented = path;
        unimplemented!("implement Loader::read");
    }
}
