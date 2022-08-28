use std::borrow::Cow;
use std::io;
use std::path::Path;

#[cfg(feature = "load-path-rubylib-native-file-system-loader")]
use artichoke_load_path::Rubylib;
use scolapasta_path::{is_explicit_relative, os_string_to_bytes};

use super::{ExtensionHook, Memory, Native};

#[derive(Debug)]
pub struct Hybrid {
    #[cfg(feature = "load-path-rubylib-native-file-system-loader")]
    rubylib: Option<Rubylib>,
    #[cfg(not(feature = "load-path-rubylib-native-file-system-loader"))]
    rubylib: Option<Native>, // hard-coded to `None`
    memory: Memory,
    native: Native,
}

impl Default for Hybrid {
    fn default() -> Self {
        Self::new()
    }
}

impl Hybrid {
    /// Create a new hybrid virtual file system.
    ///
    /// This file system allows access to the host file system with an in-memory
    /// file system mounted at [`RUBY_LOAD_PATH`].
    ///
    /// [`RUBY_LOAD_PATH`]: super::RUBY_LOAD_PATH
    #[must_use]
    pub fn new() -> Self {
        #[cfg(feature = "load-path-rubylib-native-file-system-loader")]
        let rubylib = Rubylib::new();
        #[cfg(not(feature = "load-path-rubylib-native-file-system-loader"))]
        let rubylib = None;
        let memory = Memory::new();
        let native = Native::new();
        Self {
            rubylib,
            memory,
            native,
        }
    }

    /// Check whether `path` points to a file in the virtual file system and
    /// return the absolute path if it exists.
    ///
    /// This API is infallible and will return [`None`] for non-existent paths.
    #[must_use]
    pub fn resolve_file(&self, path: &Path) -> Option<Vec<u8>> {
        if is_explicit_relative(path) {
            return self.memory.resolve_file(path).or_else(|| {
                self.native
                    .resolve_file(path)
                    .and_then(|path| os_string_to_bytes(path.into()).ok())
            });
        }
        if let Some(ref rubylib) = self.rubylib {
            rubylib
                .resolve_file(path)
                .and_then(|path| os_string_to_bytes(path.into()).ok())
                .or_else(|| {
                    self.memory.resolve_file(path).or_else(|| {
                        self.native
                            .resolve_file(path)
                            .and_then(|path| os_string_to_bytes(path.into()).ok())
                    })
                })
        } else {
            self.memory.resolve_file(path).or_else(|| {
                self.native
                    .resolve_file(path)
                    .and_then(|path| os_string_to_bytes(path.into()).ok())
            })
        }
    }

    /// Check whether `path` points to a file in the virtual file system.
    ///
    /// This API is infallible and will return `false` for non-existent paths.
    #[must_use]
    pub fn is_file(&self, path: &Path) -> bool {
        if is_explicit_relative(path) {
            return self.memory.is_file(path) || self.native.is_file(path);
        }
        if let Some(ref rubylib) = self.rubylib {
            if rubylib.is_file(path) {
                return true;
            }
        }
        self.memory.is_file(path) || self.native.is_file(path)
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
    pub fn read_file(&self, path: &Path) -> io::Result<Vec<u8>> {
        if is_explicit_relative(path) {
            return self.memory.read_file(path).or_else(|_| self.native.read_file(path));
        }
        if let Some(ref rubylib) = self.rubylib {
            rubylib
                .read_file(path)
                .or_else(|_| self.memory.read_file(path).or_else(|_| self.native.read_file(path)))
        } else {
            self.memory.read_file(path).or_else(|_| self.native.read_file(path))
        }
    }

    /// Write file contents into the virtual file system at `path`.
    ///
    /// Writes the full file contents. If any file contents already exist at
    /// `path`, they are replaced. Extension hooks are preserved.
    ///
    /// Only the [`Memory`] file system at [`RUBY_LOAD_PATH`] is writable.
    ///
    /// # Errors
    ///
    /// If access to the [`Memory`] file system returns an error, the error is
    /// returned. See [`Memory::write_file`].
    ///
    /// [`RUBY_LOAD_PATH`]: super::RUBY_LOAD_PATH
    pub fn write_file(&mut self, path: &Path, buf: Cow<'static, [u8]>) -> io::Result<()> {
        self.memory.write_file(path, buf)
    }

    /// Retrieve an extension hook for the file at `path`.
    ///
    /// This API is infallible and will return `None` for non-existent paths.
    #[must_use]
    pub fn get_extension(&self, path: &Path) -> Option<ExtensionHook> {
        self.memory.get_extension(path)
    }

    /// Write extension hook into the virtual file system at `path`.
    ///
    /// If any extension hooks already exist at `path`, they are replaced. File
    /// contents are preserved.
    ///
    /// This function writes all extensions to the virtual file system. If the
    /// given path does not map to the virtual file system, the extension is
    /// unreachable.
    ///
    /// # Errors
    ///
    /// If the given path does not resolve to the virtual file system, an error
    /// is returned.
    pub fn register_extension(&mut self, path: &Path, extension: ExtensionHook) -> io::Result<()> {
        self.memory.register_extension(path, extension)
    }

    /// Check whether a file at `path` has been required already.
    ///
    /// This API is infallible and will return `false` for non-existent paths.
    #[must_use]
    pub fn is_required(&self, path: &Path) -> Option<bool> {
        if is_explicit_relative(path) {
            if let Some(required) = self.memory.is_required(path) {
                return Some(required);
            }
            return self.native.is_required(path);
        }
        if let Some(ref rubylib) = self.rubylib {
            if let Some(required) = rubylib.is_required(path) {
                return Some(required);
            }
        }
        if let Some(required) = self.memory.is_required(path) {
            Some(required)
        } else {
            self.native.is_required(path)
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
        if is_explicit_relative(path) {
            return self
                .memory
                .mark_required(path)
                .or_else(|_| self.native.mark_required(path));
        }
        if let Some(ref mut rubylib) = self.rubylib {
            rubylib.mark_required(path).or_else(|_| {
                self.memory
                    .mark_required(path)
                    .or_else(|_| self.native.mark_required(path))
            })
        } else {
            self.memory
                .mark_required(path)
                .or_else(|_| self.native.mark_required(path))
        }
    }
}
