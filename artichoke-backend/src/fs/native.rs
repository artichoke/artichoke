use bstr::{BString, ByteSlice};
use std::borrow::Cow;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

use crate::ffi::os_str_to_bytes;
use crate::fs::{absolutize_relative_to, normalize_slashes, ExtensionHook};

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

    /// Write file contents into the virtual file system at `path`.
    ///
    /// Writes the full file contents. If any file contents already exist at
    /// `path`, they are replaced. Extension hooks are preserved.
    ///
    /// # Errors
    ///
    /// If `path` does not exist, an [`io::Error`] with error kind
    /// [`io::ErrorKind::NotFound`] is returned. See [`fs::write`] for further
    /// discussion of the error modes of this API.
    #[allow(clippy::unused_self)]
    pub fn write_file(&mut self, path: &Path, buf: &[u8]) -> io::Result<()> {
        fs::write(path, buf)
    }

    /// Retrieve an extension hook for the file at `path`.
    ///
    /// This API is infallible and will return `None` for non-existent paths.
    ///
    /// The native filesystem does not support extensions. This method always
    /// returns [`None`].
    #[must_use]
    #[allow(clippy::unused_self)]
    pub fn get_extension(&self, path: &Path) -> Option<ExtensionHook> {
        let _ = path;
        None
    }

    /// Write extension hook into the virtual file system at `path`.
    ///
    /// The native filesystem does not support extensions. All given extensions
    /// result in an error.
    ///
    /// # Errors
    ///
    /// The native filesystem does not support extensions. All given extensions
    /// result in an error with `ErrorKind::Other`.
    #[allow(clippy::unused_self)]
    pub fn register_extension(&mut self, path: &Path, extension: ExtensionHook) -> io::Result<()> {
        let _ = path;
        let _ = extension;
        Err(io::Error::new(io::ErrorKind::Other, "extensions are unsupported"))
    }

    /// Check whether a file at `path` has been required already.
    ///
    /// This API is infallible and will return `false` for non-existent paths.
    #[must_use]
    pub fn is_required(&self, path: &Path) -> bool {
        let path = if let Ok(cwd) = env::current_dir() {
            absolutize_relative_to(path, &cwd)
        } else {
            return false;
        };
        if let Ok(path) = normalize_slashes(path) {
            self.loaded_features.contains(path.as_bstr())
        } else {
            false
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
