//! Path routines for interfacing with load paths.
//!
//! This module contains functions to manipulate and configure load paths for a
//! Ruby interpreter.
//!
//! These functions are defined in terms of [`Path`] from the Rust Standard
//! Library.

use std::path::Path;

mod default;
#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

#[cfg(not(any(unix, windows)))]
use default as imp;
#[cfg(unix)]
use unix as imp;
#[cfg(windows)]
use windows as imp;

/// Directory at which Ruby sources and extensions are stored in the virtual
/// file system.
///
/// Some loaders are not backed by a physical disk. These loaders use the path
/// returned by this function as a mount point and default working directory.
///
/// # Examples
///
/// On Windows systems:
///
/// ```
/// # use std::path::Path;
/// # use mezzaluna_feature_loader::paths::memory_loader_ruby_load_path;
/// # #[cfg(windows)]
/// assert_eq!(memory_loader_ruby_load_path(), Path::new("c:/artichoke/virtual_root/src/lib"));
/// ```
///
/// On non-Windows systems:
///
/// ```
/// # use std::path::Path;
/// # use mezzaluna_feature_loader::paths::memory_loader_ruby_load_path;
/// # #[cfg(not(windows))]
/// assert_eq!(memory_loader_ruby_load_path(), Path::new("/artichoke/virtual_root/src/lib"));
/// ```
#[must_use]
pub fn memory_loader_ruby_load_path() -> &'static Path {
    if cfg!(windows) {
        Path::new("c:/artichoke/virtual_root/src/lib")
    } else {
        Path::new("/artichoke/virtual_root/src/lib")
    }
}

/// Return whether the given path starts with an explicit relative path.
///
/// Explicit relative paths start with `.` or `..` followed immediately by a
/// directory separator.
///
/// Some loaders have special handling for explicit relative paths where
/// explicit relative paths are resolved relative to the process's [current
/// working directory] rather than the load path.
///
/// # Compatibility
///
/// On Windows, if the given path contains invalid Unicode code points and
/// cannot be converted to `&str`, this function will correctly identify these
/// paths as explicit relative if their prefixes allow.
///
/// On platforms that are neither Windows nor Unix, this function may return
/// incorrect results for paths that do not contain valid UTF-8. See
/// [`Path::to_str`].
///
/// # Examples
///
/// ```
/// # use mezzaluna_feature_loader::paths::is_explicit_relative;
/// assert!(is_explicit_relative("./test/loader"));
/// assert!(is_explicit_relative("../rake/test_task"));
///
/// assert!(!is_explicit_relative("json/pure"));
/// assert!(!is_explicit_relative("/artichoke/src/json/pure"));
/// ```
///
/// # MRI C Declaration
///
/// This routine is derived from the [reference implementation] in MRI Ruby:
///
/// ```c
/// static int
/// is_explicit_relative(const char *path)
/// {
///     if (*path++ != '.') return 0;
///     if (*path == '.') path++;
///     return isdirsep(*path);
/// }
/// ```
///
/// [current working directory]: std::env::current_dir
/// [reference implementation]: https://github.com/artichoke/ruby/blob/v3_0_2/file.c#L6287-L6293
#[must_use]
pub fn is_explicit_relative<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    imp::is_explicit_relative(path)
}

/// Return whether the given byte string to treat as a path starts with an
/// explicit relative path.
///
/// Explicit relative paths start with `.` or `..` followed immediately by a
/// directory separator.
///
/// Some loaders have special handling for explicit relative paths where
/// explicit relative paths are resolved relative to the process's [current
/// working directory] rather than the load path.
///
/// # Usage
///
/// This function can be used instead of [`is_explicit_relative`] if callers
/// already have a byte string, as they likely do when manipulating a Ruby
/// [`String`][ruby-string].
///
/// # Compatibility
///
/// Since this function operates on bytes, it is guaranteed to give a correct
/// boolean answer to whether a path is explicit relative on all platforms.
///
/// # Examples
///
/// ```
/// # use mezzaluna_feature_loader::paths::is_explicit_relative_bytes;
/// assert!(is_explicit_relative_bytes(b"./test/loader"));
/// assert!(is_explicit_relative_bytes(b"../rake/test_task"));
///
/// assert!(!is_explicit_relative_bytes(b"json/pure"));
/// assert!(!is_explicit_relative_bytes(b"/artichoke/src/json/pure"));
/// ```
///
/// # MRI C Declaration
///
/// This routine is derived from the [reference implementation] in MRI Ruby:
///
/// ```c
/// static int
/// is_explicit_relative(const char *path)
/// {
///     if (*path++ != '.') return 0;
///     if (*path == '.') path++;
///     return isdirsep(*path);
/// }
/// ```
///
/// [current working directory]: std::env::current_dir
/// [ruby-string]: https://ruby-doc.org/core-2.6.3/String.html
/// [reference implementation]: https://github.com/artichoke/ruby/blob/v3_0_2/file.c#L6287-L6293
#[must_use]
pub fn is_explicit_relative_bytes<P: AsRef<[u8]>>(path: P) -> bool {
    let path = path.as_ref();
    default::is_explicit_relative_bytes(path)
}
