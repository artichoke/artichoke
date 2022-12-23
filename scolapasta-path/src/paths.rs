//! Path routines for interfacing with load paths.
//!
//! This module contains functions to manipulate and configure load paths for a
//! Ruby interpreter.
//!
//! These functions are defined in terms of [`Path`] from the Rust Standard
//! Library.

use std::path::{Component, Path, PathBuf};

mod default;
#[cfg(any(unix, target_os = "wasi"))]
mod unix_wasi;
#[cfg(windows)]
mod windows;

#[cfg(not(any(unix, windows, target_os = "wasi")))]
use default as imp;
#[cfg(any(unix, target_os = "wasi"))]
use unix_wasi as imp;
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
/// # use scolapasta_path::memory_loader_ruby_load_path;
/// # #[cfg(windows)]
/// assert_eq!(
///     memory_loader_ruby_load_path(),
///     Path::new("c:/artichoke/virtual_root/src/lib")
/// );
/// ```
///
/// On non-Windows systems:
///
/// ```
/// # use std::path::Path;
/// # use scolapasta_path::memory_loader_ruby_load_path;
/// # #[cfg(not(windows))]
/// assert_eq!(
///     memory_loader_ruby_load_path(),
///     Path::new("/artichoke/virtual_root/src/lib")
/// );
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
/// Some Ruby source loaders have special handling for explicit relative paths
/// where explicit relative paths are resolved relative to the process's [current
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
/// # use scolapasta_path::is_explicit_relative;
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
    imp::is_explicit_relative(path.as_ref())
}

/// Return whether the given byte string to treat as a path starts with an
/// explicit relative path.
///
/// Explicit relative paths start with `.` or `..` followed immediately by a
/// directory separator.
///
/// Some Ruby source loaders have special handling for explicit relative paths
/// where explicit relative paths are resolved relative to the process's [current
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
/// Boolean answer to whether a path is explicit relative on all platforms.
///
/// # Examples
///
/// ```
/// # use scolapasta_path::is_explicit_relative_bytes;
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
/// [ruby-string]: https://ruby-doc.org/core-3.1.2/String.html
/// [reference implementation]: https://github.com/artichoke/ruby/blob/v3_0_2/file.c#L6287-L6293
#[must_use]
pub fn is_explicit_relative_bytes<P: AsRef<[u8]>>(path: P) -> bool {
    let path = path.as_ref();
    default::is_explicit_relative_bytes(path)
}

/// Normalize path separators to all be `/`.
///
/// This function is a no-op on all non-Windows platforms. On Windows, this
/// function will convert `\` separators to `/` if the given [`PathBuf`] is
/// valid UTF-8.
///
/// # Errors
///
/// On Unix platforms, this function is infallible. On all other platforms,
/// including Windows, if the given [`PathBuf`] is not valid UTF-8, the original
/// `PathBuf` is returned as an error. See [`Path::to_str`] for details.
pub fn normalize_slashes(path: PathBuf) -> Result<Vec<u8>, PathBuf> {
    imp::normalize_slashes(path)
}

/// Translate a relative path into an absolute path, using a secondary path
/// as the frame of reference.
pub fn absolutize_relative_to<T, U>(path: T, cwd: U) -> PathBuf
where
    T: AsRef<Path>,
    U: AsRef<Path>,
{
    absolutize_relative_to_inner(path.as_ref(), cwd.as_ref())
}

fn absolutize_relative_to_inner(path: &Path, cwd: &Path) -> PathBuf {
    let mut iter = path.components().peekable();
    let hint = iter.size_hint();
    let (mut components, cwd_is_relative) = if let Some(Component::RootDir) = iter.peek() {
        (Vec::with_capacity(hint.1.unwrap_or(hint.0)), false)
    } else {
        let mut components = cwd.components().map(Component::as_os_str).collect::<Vec<_>>();
        components.reserve(hint.1.unwrap_or(hint.0));
        (components, cwd.is_relative())
    };
    for component in iter {
        match component {
            Component::CurDir => {}
            Component::ParentDir if cwd_is_relative => {
                components.pop();
            }
            Component::ParentDir => {
                components.pop();
                if components.is_empty() {
                    components.push(Component::RootDir.as_os_str());
                }
            }
            c => {
                components.push(c.as_os_str());
            }
        }
    }
    components.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::absolutize_relative_to;

    #[test]
    fn absolutize_absolute_path() {
        let path = Path::new("/foo/bar");
        let cwd = Path::new("/home/artichoke");
        assert_eq!(absolutize_relative_to(path, cwd), path);
        let cwd = Path::new("relative/path");
        assert_eq!(absolutize_relative_to(path, cwd), path);
    }

    #[test]
    fn absolutize_absolute_path_dedot_current_dir() {
        let path = Path::new("/././foo/./bar/./././.");
        let cwd = Path::new("/home/artichoke");
        assert_eq!(absolutize_relative_to(path, cwd), Path::new("/foo/bar"));
        let cwd = Path::new("relative/path");
        assert_eq!(absolutize_relative_to(path, cwd), Path::new("/foo/bar"));
    }

    #[test]
    fn absolutize_absolute_path_dedot_parent_dir() {
        let path = Path::new("/foo/bar/..");
        let cwd = Path::new("/home/artichoke");
        assert_eq!(absolutize_relative_to(path, cwd), Path::new("/foo"));
        let cwd = Path::new("relative/path");
        assert_eq!(absolutize_relative_to(path, cwd), Path::new("/foo"));

        let path = Path::new("/foo/../../../../bar/../../../");
        let cwd = Path::new("/home/artichoke");
        assert_eq!(absolutize_relative_to(path, cwd), Path::new("/"));
        let cwd = Path::new("relative/path");
        assert_eq!(absolutize_relative_to(path, cwd), Path::new("/"));

        let path = Path::new("/foo/../../../../bar/../../../boom/baz");
        let cwd = Path::new("/home/artichoke");
        assert_eq!(absolutize_relative_to(path, cwd), Path::new("/boom/baz"));
        let cwd = Path::new("relative/path");
        assert_eq!(absolutize_relative_to(path, cwd), Path::new("/boom/baz"));
    }

    #[test]
    fn absolutize_relative_path() {
        let path = Path::new("foo/bar");
        let cwd = Path::new("/home/artichoke");
        assert_eq!(absolutize_relative_to(path, cwd), Path::new("/home/artichoke/foo/bar"));
        let cwd = Path::new("relative/path");
        assert_eq!(absolutize_relative_to(path, cwd), Path::new("relative/path/foo/bar"));
    }

    #[test]
    fn absolutize_relative_path_dedot_current_dir() {
        let path = Path::new("././././foo/./bar/./././.");
        let cwd = Path::new("/home/artichoke");
        assert_eq!(absolutize_relative_to(path, cwd), Path::new("/home/artichoke/foo/bar"));
        let cwd = Path::new("relative/path");
        assert_eq!(absolutize_relative_to(path, cwd), Path::new("relative/path/foo/bar"));
    }

    #[test]
    #[cfg(unix)]
    fn absolutize_relative_path_dedot_parent_dir_unix() {
        let path = Path::new("foo/bar/..");
        let cwd = Path::new("/home/artichoke");
        let absolute = absolutize_relative_to(path, cwd);
        assert_eq!(absolute, Path::new("/home/artichoke/foo"));
        let cwd = Path::new("relative/path");
        let absolute = absolutize_relative_to(path, cwd);
        assert_eq!(absolute, Path::new("relative/path/foo"));

        let path = Path::new("foo/../../../../bar/../../../");
        let cwd = Path::new("/home/artichoke");
        let absolute = absolutize_relative_to(path, cwd);
        assert_eq!(absolute, Path::new("/"));
        let cwd = Path::new("relative/path");
        let absolute = absolutize_relative_to(path, cwd);
        assert_eq!(absolute, Path::new(""));

        let path = Path::new("foo/../../../../bar/../../../boom/baz");
        let cwd = Path::new("/home/artichoke");
        let absolute = absolutize_relative_to(path, cwd);
        assert_eq!(absolute, Path::new("/boom/baz"));
        let cwd = Path::new("relative/path");
        let absolute = absolutize_relative_to(path, cwd);
        assert_eq!(absolute, Path::new("boom/baz"));
    }

    #[test]
    #[cfg(windows)]
    fn absolutize_relative_path_dedot_parent_dir_windows_forward_slash() {
        let path = Path::new("foo/bar/..");
        let cwd = Path::new("C:/Users/artichoke");
        let absolute = absolutize_relative_to(path, cwd);
        assert_eq!(absolute, Path::new("C:/Users/artichoke/foo"));
        let cwd = Path::new("relative/path");
        let absolute = absolutize_relative_to(path, cwd);
        assert_eq!(absolute, Path::new("relative/path/foo"));

        let path = Path::new("foo/../../../../bar/../../../");
        let cwd = Path::new("C:/Users/artichoke");
        let absolute = absolutize_relative_to(path, cwd);
        assert_eq!(absolute, Path::new("/"));
        let cwd = Path::new("relative/path");
        let absolute = absolutize_relative_to(path, cwd);
        assert_eq!(absolute, Path::new(""));

        let path = Path::new("foo/../../../../bar/../../../boom/baz");
        let cwd = Path::new("C:/Users/artichoke");
        let absolute = absolutize_relative_to(path, cwd);
        assert_eq!(absolute, Path::new("/boom/baz"));
        let cwd = Path::new("relative/path");
        let absolute = absolutize_relative_to(path, cwd);
        assert_eq!(absolute, Path::new("boom/baz"));
    }

    #[test]
    #[cfg(windows)]
    fn absolutize_relative_path_dedot_parent_dir_windows_backward_slash() {
        let path = Path::new(r"foo\bar\..");
        let cwd = Path::new(r"C:\Users\artichoke");
        let absolute = absolutize_relative_to(path, cwd);
        assert_eq!(absolute, Path::new("C:/Users/artichoke/foo"));
        let cwd = Path::new(r"relative\path");
        let absolute = absolutize_relative_to(path, cwd);
        assert_eq!(absolute, Path::new("relative/path/foo"));

        let path = Path::new(r"foo\..\..\..\..\bar\..\..\..\");
        let cwd = Path::new(r"C:\Users\artichoke");
        let absolute = absolutize_relative_to(path, cwd);
        assert_eq!(absolute, Path::new("/"));
        let cwd = Path::new(r"relative\path");
        let absolute = absolutize_relative_to(path, cwd);
        assert_eq!(absolute, Path::new(""));

        let path = Path::new(r"foo\..\..\..\..\bar\..\..\..\boom\baz");
        let cwd = Path::new(r"C:\Users\artichoke");
        let absolute = absolutize_relative_to(path, cwd);
        assert_eq!(absolute, Path::new("/boom/baz"));
        let cwd = Path::new(r"relative\path");
        let absolute = absolutize_relative_to(path, cwd);
        assert_eq!(absolute, Path::new("boom/baz"));
    }
}
