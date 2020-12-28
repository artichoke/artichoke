//! Virtual filesystem.
//!
//! Artichoke proxies all filesystem access through a virtual filesystem. The
//! filesystem can store Ruby sources and [extension hooks](ExtensionHook) in
//! memory and will support proxying to the host filesystem for reads and
//! writes.
//!
//! Artichoke uses the virtual filesystem to track metadata about loaded
//! features.
//!
//! Artichoke has several virtual filesystem implementations. Only some of them
//! support reading from the system fs.

use std::borrow::Cow;
use std::fmt;
use std::io;
use std::path::{Component, Path, PathBuf};

use crate::error::Error;
use crate::Artichoke;

pub mod hybrid;
pub mod memory;
pub mod native;

/// Directory at which Ruby sources and extensions are stored in the virtual
/// filesystem.
///
/// `RUBY_LOAD_PATH` is the default current working directory for
/// [`Memory`](memory::Memory) filesystems.
///
/// [`Hybrid`](hybrid::Hybrid) filesystems mount the `Memory` filessytem at
/// `RUBY_LOAD_PATH`.
pub const RUBY_LOAD_PATH: &str = "/src/lib";

/// Function type for extension hooks stored in the virtual filesystem.
///
/// This signature is equivalent to the signature for `File::require` as defined
/// by the `artichoke-backend` implementation of
/// [`LoadSources`](crate::core::LoadSources).
pub type ExtensionHook = fn(&mut Artichoke) -> Result<(), Error>;

#[must_use]
#[cfg(all(feature = "native-filesystem-access", not(any(test, doctest))))]
pub fn filesystem() -> Box<dyn Filesystem> {
    let fs = hybrid::Hybrid::default();
    Box::new(fs)
}

#[must_use]
#[cfg(not(any(feature = "native-filesystem-access", test, doctest)))]
pub fn filesystem() -> Box<dyn Filesystem> {
    let fs = memory::Memory::default();
    Box::new(fs)
}

#[must_use]
#[cfg(any(doctest, test))]
pub fn filesystem() -> Box<dyn Filesystem> {
    let fs = memory::Memory::default();
    Box::new(fs)
}

/// Filesystem APIs required by an Artichoke interpreter.
pub trait Filesystem: fmt::Debug {
    /// Check whether `path` points to a file in the virtual filesystem.
    ///
    /// This API is infallible and will return `false` for non-existent paths.
    fn is_file(&self, path: &Path) -> bool;

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
    fn read_file(&self, path: &Path) -> io::Result<Cow<'_, [u8]>>;

    /// Write file contents into the virtual file system at `path`.
    ///
    /// Writes the full file contents. If any file contents already exist at
    /// `path`, they are replaced. Extension hooks are preserved.
    ///
    /// # Errors
    ///
    /// This API is currently infallible but returns [`io::Result`] to reserve
    /// the ability to return errors in the future.
    fn write_file(&mut self, path: &Path, buf: Cow<'static, [u8]>) -> io::Result<()>;

    /// Retrieve an extension hook for the file at `path`.
    ///
    /// This API is infallible and will return `None` for non-existent paths.
    fn get_extension(&self, path: &Path) -> Option<ExtensionHook>;

    /// Write extension hook into the virtual file system at `path`.
    ///
    /// If any extension hooks already exist at `path`, they are replaced. File
    /// contents are preserved.
    ///
    /// # Errors
    ///
    /// This API is currently infallible but returns [`io::Result`] to reserve
    /// the ability to return errors in the future.
    fn register_extension(&mut self, path: &Path, extension: ExtensionHook) -> io::Result<()>;

    /// Check whether a file at `path` has been required already.
    ///
    /// This API is infallible and will return `false` for non-existent paths.
    fn is_required(&self, path: &Path) -> bool;

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
    fn mark_required(&mut self, path: &Path) -> io::Result<()>;
}

impl Default for Box<dyn Filesystem> {
    fn default() -> Self {
        filesystem()
    }
}

fn absolutize_relative_to<T, U>(path: T, cwd: U) -> PathBuf
where
    T: AsRef<Path>,
    U: AsRef<Path>,
{
    let mut iter = path.as_ref().components().peekable();
    let hint = iter.size_hint();
    let (mut components, cwd_is_relative) = if let Some(Component::RootDir) = iter.peek() {
        (Vec::with_capacity(hint.1.unwrap_or(hint.0)), false)
    } else {
        let mut components = cwd.as_ref().components().map(Component::as_os_str).collect::<Vec<_>>();
        components.reserve(hint.1.unwrap_or(hint.0));
        (components, cwd.as_ref().is_relative())
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
#[allow(clippy::shadow_unrelated)]
mod tests {
    use std::path::Path;

    use super::absolutize_relative_to;

    #[test]
    fn absolutize_absolute_path() {
        let path = Path::new("/foo/bar");
        let cwd = Path::new("/home/artichoke");
        assert_eq!(absolutize_relative_to(&path, cwd), path);
        let cwd = Path::new("relative/path");
        assert_eq!(absolutize_relative_to(&path, cwd), path);
    }

    #[test]
    fn absolutize_absolute_path_dedot_current_dir() {
        let path = Path::new("/././foo/./bar/./././.");
        let cwd = Path::new("/home/artichoke");
        assert_eq!(absolutize_relative_to(&path, cwd), Path::new("/foo/bar"));
        let cwd = Path::new("relative/path");
        assert_eq!(absolutize_relative_to(&path, cwd), Path::new("/foo/bar"));
    }

    #[test]
    fn absolutize_absolute_path_dedot_parent_dir() {
        let path = Path::new("/foo/bar/..");
        let cwd = Path::new("/home/artichoke");
        assert_eq!(absolutize_relative_to(&path, cwd), Path::new("/foo"));
        let cwd = Path::new("relative/path");
        assert_eq!(absolutize_relative_to(&path, cwd), Path::new("/foo"));

        let path = Path::new("/foo/../../../../bar/../../../");
        let cwd = Path::new("/home/artichoke");
        assert_eq!(absolutize_relative_to(&path, cwd), Path::new("/"));
        let cwd = Path::new("relative/path");
        assert_eq!(absolutize_relative_to(&path, cwd), Path::new("/"));

        let path = Path::new("/foo/../../../../bar/../../../boom/baz");
        let cwd = Path::new("/home/artichoke");
        assert_eq!(absolutize_relative_to(&path, cwd), Path::new("/boom/baz"));
        let cwd = Path::new("relative/path");
        assert_eq!(absolutize_relative_to(&path, cwd), Path::new("/boom/baz"));
    }

    #[test]
    fn absolutize_relative_path() {
        let path = Path::new("foo/bar");
        let cwd = Path::new("/home/artichoke");
        assert_eq!(absolutize_relative_to(&path, cwd), Path::new("/home/artichoke/foo/bar"));
        let cwd = Path::new("relative/path");
        assert_eq!(absolutize_relative_to(&path, cwd), Path::new("relative/path/foo/bar"));
    }

    #[test]
    fn absolutize_relative_path_dedot_current_dir() {
        let path = Path::new("././././foo/./bar/./././.");
        let cwd = Path::new("/home/artichoke");
        assert_eq!(absolutize_relative_to(&path, cwd), Path::new("/home/artichoke/foo/bar"));
        let cwd = Path::new("relative/path");
        assert_eq!(absolutize_relative_to(&path, cwd), Path::new("relative/path/foo/bar"));
    }

    #[test]
    #[cfg(unix)]
    fn absolutize_relative_path_dedot_parent_dir_unix() {
        let path = Path::new("foo/bar/..");
        let cwd = Path::new("/home/artichoke");
        let absolute = absolutize_relative_to(&path, cwd);
        assert_eq!(absolute, Path::new("/home/artichoke/foo"));
        let cwd = Path::new("relative/path");
        let absolute = absolutize_relative_to(&path, cwd);
        assert_eq!(absolute, Path::new("relative/path/foo"));

        let path = Path::new("foo/../../../../bar/../../../");
        let cwd = Path::new("/home/artichoke");
        let absolute = absolutize_relative_to(&path, cwd);
        assert_eq!(absolute, Path::new("/"));
        let cwd = Path::new("relative/path");
        let absolute = absolutize_relative_to(&path, cwd);
        assert_eq!(absolute, Path::new(""));

        let path = Path::new("foo/../../../../bar/../../../boom/baz");
        let cwd = Path::new("/home/artichoke");
        let absolute = absolutize_relative_to(&path, cwd);
        assert_eq!(absolute, Path::new("/boom/baz"));
        let cwd = Path::new("relative/path");
        let absolute = absolutize_relative_to(&path, cwd);
        assert_eq!(absolute, Path::new("boom/baz"));
    }

    #[test]
    #[cfg(windows)]
    fn absolutize_relative_path_dedot_parent_dir_windows_forward_slash() {
        let path = Path::new("foo/bar/..");
        let cwd = Path::new("C:/Users/artichoke");
        let absolute = absolutize_relative_to(&path, cwd);
        assert_eq!(absolute, Path::new("C:/Users/artichoke/foo"));
        let cwd = Path::new("relative/path");
        let absolute = absolutize_relative_to(&path, cwd);
        assert_eq!(absolute, Path::new("relative/path/foo"));

        let path = Path::new("foo/../../../../bar/../../../");
        let cwd = Path::new("C:/Users/artichoke");
        let absolute = absolutize_relative_to(&path, cwd);
        assert_eq!(absolute, Path::new("/"));
        let cwd = Path::new("relative/path");
        let absolute = absolutize_relative_to(&path, cwd);
        assert_eq!(absolute, Path::new(""));

        let path = Path::new("foo/../../../../bar/../../../boom/baz");
        let cwd = Path::new("C:/Users/artichoke");
        let absolute = absolutize_relative_to(&path, cwd);
        assert_eq!(absolute, Path::new("/boom/baz"));
        let cwd = Path::new("relative/path");
        let absolute = absolutize_relative_to(&path, cwd);
        assert_eq!(absolute, Path::new("boom/baz"));
    }

    #[test]
    #[cfg(windows)]
    fn absolutize_relative_path_dedot_parent_dir_windows_backward_slash() {
        let path = Path::new(r"foo\bar\..");
        let cwd = Path::new(r"C:\Users\artichoke");
        let absolute = absolutize_relative_to(&path, cwd);
        assert_eq!(absolute, Path::new("C:/Users/artichoke/foo"));
        let cwd = Path::new(r"relative\path");
        let absolute = absolutize_relative_to(&path, cwd);
        assert_eq!(absolute, Path::new("relative/path/foo"));

        let path = Path::new(r"foo\..\..\..\..\bar\..\..\..\");
        let cwd = Path::new(r"C:\Users\artichoke");
        let absolute = absolutize_relative_to(&path, cwd);
        assert_eq!(absolute, Path::new("/"));
        let cwd = Path::new(r"relative\path");
        let absolute = absolutize_relative_to(&path, cwd);
        assert_eq!(absolute, Path::new(""));

        let path = Path::new(r"foo\..\..\..\..\bar\..\..\..\boom\baz");
        let cwd = Path::new(r"C:\Users\artichoke");
        let absolute = absolutize_relative_to(&path, cwd);
        assert_eq!(absolute, Path::new("/boom/baz"));
        let cwd = Path::new(r"relative\path");
        let absolute = absolutize_relative_to(&path, cwd);
        assert_eq!(absolute, Path::new("boom/baz"));
    }
}
