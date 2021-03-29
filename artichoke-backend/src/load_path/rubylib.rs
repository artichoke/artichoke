use bstr::{BString, ByteSlice};
use std::borrow::Cow;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use super::{absolutize_relative_to, normalize_slashes};
use crate::platform_string::os_string_to_bytes;

#[derive(Debug, PartialEq, Eq)]
pub struct Rubylib {
    load_paths: Vec<PathBuf>,
    loaded_features: HashSet<BString>,
}

impl Rubylib {
    /// Create a new native virtual filesystem that searches the filesystem for
    /// Ruby sources at the paths specified by the `RUBYLIB` environment
    /// variable.
    ///
    /// This filesystem grants access to the host filesystem.
    #[must_use]
    pub fn new() -> Option<Self> {
        let rubylib = env::var_os("RUBYLIB")?;
        let cwd = env::current_dir().ok()?;
        let load_paths = env::split_paths(&rubylib)
            .map(|load_path| absolutize_relative_to(&load_path, &cwd))
            .collect::<Vec<_>>();
        if load_paths.is_empty() {
            return None;
        }
        Some(Self {
            load_paths,
            loaded_features: HashSet::default(),
        })
    }

    /// Check whether `path` points to a file in the virtual filesystem and
    /// return the absolute path if it exists.
    ///
    /// This API is infallible and will return [`None`] for non-existent paths.
    #[must_use]
    pub fn resolve_file(&self, path: &Path) -> Option<Vec<u8>> {
        for load_path in &self.load_paths {
            let path = absolutize_relative_to(path, &load_path);
            if path.exists() {
                return os_string_to_bytes(path.into()).ok();
            }
        }
        None
    }

    /// Check whether `path` points to a file in the virtual filesystem.
    ///
    /// This API is infallible and will return `false` for non-existent paths.
    #[must_use]
    #[allow(clippy::unused_self)]
    pub fn is_file(&self, path: &Path) -> bool {
        for load_path in &self.load_paths {
            let path = absolutize_relative_to(path, &load_path);
            if let Ok(metadata) = fs::metadata(path) {
                return !metadata.is_dir();
            }
        }
        false
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
        for load_path in &self.load_paths {
            let path = absolutize_relative_to(path, &load_path);
            if let Ok(contents) = fs::read(path) {
                return Ok(contents.into());
            }
        }
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "path not found in RUBYLIB load paths",
        ))
    }

    /// Check whether a file at `path` has been required already.
    ///
    /// This API is infallible and will return `false` for non-existent paths.
    #[must_use]
    pub fn is_required(&self, path: &Path) -> bool {
        for load_path in &self.load_paths {
            let path = absolutize_relative_to(path, &load_path);
            if let Ok(path) = normalize_slashes(path) {
                return self.loaded_features.contains(path.as_bstr());
            }
        }
        false
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
        for load_path in &self.load_paths {
            let path = absolutize_relative_to(path, &load_path);
            if path.exists() {
                let path = normalize_slashes(path).map_err(|err| io::Error::new(io::ErrorKind::NotFound, err))?;
                self.loaded_features.insert(path.into());
                return Ok(());
            }
        }
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "file not found in RUBYLIB load path",
        ))
    }
}
