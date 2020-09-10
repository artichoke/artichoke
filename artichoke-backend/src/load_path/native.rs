use bstr::{BString, ByteSlice};
use std::borrow::Cow;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

use super::{absolutize_relative_to, normalize_slashes};
use crate::platform_string::os_str_to_bytes;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Native {
    loaded_features: HashSet<BString>,
}

impl Native {
    /// Create a new native virtual filesystem.
    ///
    /// This filesystem grants access to the host filesystem.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Check whether `path` points to a file in the virtual filesystem and
    /// return the absolute path if it exists.
    ///
    /// This API is infallible and will return [`None`] for non-existent paths.
    #[must_use]
    #[allow(clippy::unused_self)]
    pub fn resolve_file(&self, path: &Path) -> Option<Vec<u8>> {
        if path.exists() {
            let file = os_str_to_bytes(path.as_os_str()).ok()?;
            Some(file.to_vec())
        } else {
            None
        }
    }

    /// Check whether `path` points to a file in the virtual filesystem.
    ///
    /// This API is infallible and will return `false` for non-existent paths.
    #[must_use]
    #[allow(clippy::unused_self)]
    pub fn is_file(&self, path: &Path) -> bool {
        if let Ok(metadata) = fs::metadata(path) {
            !metadata.is_dir()
        } else {
            false
        }
    }

    /// Read file contents for the file at `path`.
    ///
    /// Returns a byte slice of complete file contents. If `path` is relative,
    /// it is absolutized relative to the current working directory of the
    /// virtual file system.
    ///
    /// # Errors
    ///
    /// If `path` does not exist, an [`io::Error`] with error kind
    /// [`io::ErrorKind::NotFound`] is returned.
    #[allow(clippy::unused_self)]
    pub fn read_file(&self, path: &Path) -> io::Result<Cow<'_, [u8]>> {
        Ok(fs::read(path)?.into())
    }

    /// Check whether a file at `path` has been required already.
    ///
    /// This API is infallible and will return `false` for non-existent paths.
    #[must_use]
    pub fn is_required(&self, path: &Path) -> Option<bool> {
        let path = if let Ok(cwd) = env::current_dir() {
            absolutize_relative_to(path, &cwd)
        } else {
            return None;
        };
        if let Ok(path) = normalize_slashes(path) {
            Some(self.loaded_features.contains(path.as_bstr()))
        } else {
            None
        }
    }

    /// Mark a source at `path` as required on the interpreter.
    ///
    /// This metadata is used by `Kernel#require` and friends to enforce that
    /// Ruby sources are only loaded into the interpreter once to limit side
    /// effects.
    ///
    /// # Errors
    ///
    /// If `path` does not exist, an [`io::Error`] with error kind
    /// [`io::ErrorKind::NotFound`] is returned.
    pub fn mark_required(&mut self, path: &Path) -> io::Result<()> {
        let cwd = env::current_dir()?;
        let path = absolutize_relative_to(path, &cwd);
        let path = normalize_slashes(path).map_err(|err| io::Error::new(io::ErrorKind::NotFound, err))?;
        self.loaded_features.insert(path.into());
        Ok(())
    }
}
