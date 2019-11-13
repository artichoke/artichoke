/* Copyright (c) 2018 Garrett Berg, vitiral@gmail.com
 *
 * Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
 * http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
 * http://opensource.org/licenses/MIT>, at your option. This file may not be
 * copied, modified, or distributed except according to those terms.
 */

//! The absolute path type.

use std::borrow::Borrow;
use std::error;
use std::ffi;
use std::fmt;
use std::io;
use std::path::{Component, Path, PathBuf, PrefixComponent};
use std::sync::Arc;

pub type Result<T> = ::std::result::Result<T, Error>;

pub struct Error {
    io_err: io::Error,
    action: String,
    path: Arc<PathBuf>,
}

impl Error {
    /// Create a new error when the path and action are known.
    pub fn new(io_err: io::Error, action: &str, path: Arc<PathBuf>) -> Self {
        Self {
            io_err,
            action: action.into(),
            path,
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error<{}>", self)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} when {} {}",
            self.io_err,
            self.action,
            self.path.display()
        )
    }
}

impl Error {
    /// Returns the path associated with this error.
    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }

    /// Returns the `std::io::Error` associated with this errors.
    pub fn io_error(&self) -> &io::Error {
        &self.io_err
    }

    /// Returns the action being performed when this error occured.
    pub fn action(&self) -> &str {
        &self.action
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.io_err.description()
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        Some(&self.io_err)
    }
}

impl From<Error> for io::Error {
    fn from(err: Error) -> Self {
        Self::new(err.io_err.kind(), err)
    }
}

/// Converts any [`PrefixComponent`] into verbatim ("extended-length") form.
fn make_verbatim_prefix(prefix: &PrefixComponent<'_>) -> Result<PathBuf> {
    let path_prefix = Path::new(prefix.as_os_str());

    if prefix.kind().is_verbatim() {
        // This prefix already uses the extended-length
        // syntax, so we can use it as-is.
        Ok(path_prefix.to_path_buf())
    } else {
        // This prefix needs canonicalization.
        let res = path_prefix
            .canonicalize()
            .map_err(|e| Error::new(e, "canonicalizing", path_prefix.to_path_buf().into()))?;
        Ok(res)
    }
}

/// Pops the last component from path, returning an error for a root path.
fn pop_or_error(path: &mut PathBuf) -> ::std::result::Result<(), io::Error> {
    if path.pop() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, ".. consumed root"))
    }
}

/// An absolute (not _necessarily_ [canonicalized][1]) path that may or may not
/// exist.
///
/// [1]: https://doc.rust-lang.org/std/path/struct.Path.html?search=#method.canonicalize
#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[allow(clippy::module_name_repetitions)]
pub struct PathAbs(Arc<PathBuf>);

impl PathAbs {
    /// Construct an absolute path from an arbitrary (absolute or relative) one.
    ///
    /// This is different from [`canonicalize`](Path::canonicalize) in that it
    /// _preserves_ symlinks and the destination may or may not exist.
    ///
    /// This function will:
    /// - Resolve relative paths against the current working directory.
    /// - Strip any `.` components (`/a/./c` -> `/a/c`)
    /// - Resolve `..` _semantically_ (not using the file system). So,
    ///   `a/b/c/../d => a/b/d` will _always_ be true regardless of symlinks. If
    ///   you want symlinks correctly resolved, use `canonicalize()` instead.
    pub fn new<P: AsRef<Path>, D: AsRef<Path>>(path: P, cwd: D) -> Result<Self> {
        let path = Arc::new(path.as_ref().to_path_buf());
        let mut res = PathBuf::new();

        for each in path.components() {
            match each {
                Component::Prefix(p) => {
                    // We don't care what's already in res, we can entirely
                    // replace it..
                    res = make_verbatim_prefix(&p)?;
                }

                Component::RootDir => {
                    if cfg!(windows) {
                        // In an ideal world, we would say
                        //
                        //  res = std::fs::canonicalize(each)?;
                        //
                        // ...to get a properly canonicalized path.
                        // Unfortunately, Windows cannot canonicalize `\` if
                        // the current directory happens to use extended-length
                        // syntax (like `\\?\C:\Windows`), so we'll have to do
                        // it manually: initialize `res` with the current
                        // working directory (whatever it is), and truncate it
                        // to its prefix by pushing `\`.
                        if res.as_os_str().is_empty() {
                            // res has not been initialized, let's initialize it
                            // to the supplied current working directory.
                            res = cwd.as_ref().to_path_buf();
                        }
                        res.push(each);
                    } else {
                        // On other platforms, a root path component is always
                        // absolute so we can replace whatever's in res.
                        res = Path::new(&each).to_path_buf();
                    }
                }

                // This does nothing and can be ignored.
                Component::CurDir => (),

                Component::ParentDir => {
                    // A parent component is always relative to some existing
                    // path.
                    if res.as_os_str().is_empty() {
                        // res has not been initialized, let's initialize it to
                        // the supplied current working directory.
                        res = cwd.as_ref().to_path_buf();
                    }
                    pop_or_error(&mut res)
                        .map_err(|e| Error::new(e, "resolving absolute", path.clone()))?;
                }

                Component::Normal(c) => {
                    // A normal component is always relative to some existing
                    // path.
                    if res.as_os_str().is_empty() {
                        // res has not been initialized, let's initialize it to
                        // the supplied current working directory.
                        res = cwd.as_ref().to_path_buf();
                    }
                    res.push(c);
                }
            }
        }

        Ok(Self(Arc::new(res)))
    }

    /// Create a PathAbs unchecked.
    ///
    /// This is mostly used for constructing during tests, or if the path was previously validated.
    /// This is effectively the same as a `Arc<PathBuf>`.
    ///
    /// > Note: This is memory safe, so is not marked `unsafe`. However, it could cause
    /// > panics in some methods if the path was not properly validated.
    pub fn new_unchecked<P: Into<Arc<PathBuf>>>(path: P) -> Self {
        Self(path.into())
    }

    /// Return a reference to a basic `std::path::Path`
    pub fn as_path(&self) -> &Path {
        self.as_ref()
    }
}

impl fmt::Debug for PathAbs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<ffi::OsStr> for PathAbs {
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.0.as_ref().as_ref()
    }
}

impl AsRef<Path> for PathAbs {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl AsRef<PathBuf> for PathAbs {
    fn as_ref(&self) -> &PathBuf {
        self.0.as_ref()
    }
}

impl Borrow<Path> for PathAbs {
    fn borrow(&self) -> &Path {
        self.as_ref()
    }
}

impl Borrow<PathBuf> for PathAbs {
    fn borrow(&self) -> &PathBuf {
        self.as_ref()
    }
}

impl<'a> Borrow<Path> for &'a PathAbs {
    fn borrow(&self) -> &Path {
        self.as_ref()
    }
}

impl<'a> Borrow<PathBuf> for &'a PathAbs {
    fn borrow(&self) -> &PathBuf {
        self.as_ref()
    }
}

impl From<PathAbs> for Arc<PathBuf> {
    fn from(path: PathAbs) -> Self {
        path.0
    }
}

impl From<PathAbs> for PathBuf {
    fn from(path: PathAbs) -> Self {
        match Arc::try_unwrap(path.0) {
            Ok(p) => p,
            Err(inner) => inner.as_ref().clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::io;
    use std::path;

    use super::PathAbs;

    fn setup() {
        #[cfg(windows)]
        std::env::set_current_dir(r"C:\").expect("Could not change to a regular directory");

        // For cfg(unix), we're always in a regular directory, so we don't need
        // to do anything special.
    }

    #[test]
    fn absolute_path_is_idempotent() {
        setup();
        // The current_dir() result is always absolute,
        // so absolutizing it should not change it.

        let actual = PathAbs::new(env::current_dir().unwrap(), "").unwrap();
        let expected = env::current_dir().unwrap().canonicalize().unwrap();

        assert_eq!(actual.as_path(), expected.as_path());
    }

    #[test]
    fn absolute_path_removes_currentdir_component() {
        setup();
        let actual = PathAbs::new("foo/./bar", "").unwrap();
        let expected = PathAbs::new("foo/bar", "").unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn absolute_path_removes_empty_component() {
        setup();
        let actual = PathAbs::new("foo//bar", "").unwrap();
        let expected = PathAbs::new("foo/bar", "").unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn absolute_path_interprets_relative_to_current_directory() {
        setup();
        let actual = PathAbs::new("foo", env::current_dir().unwrap()).unwrap();
        let expected = PathAbs::new(env::current_dir().unwrap().join("foo"), "").unwrap();

        assert_eq!(actual, expected);
    }

    #[cfg(unix)]
    mod unix {
        use super::*;

        #[test]
        fn absolute_path_need_not_exist() {
            setup();

            // It's not likely this path would exist, but let's be sure.
            let raw_path = path::Path::new("/does/not/exist");
            assert_eq!(
                raw_path.metadata().unwrap_err().kind(),
                io::ErrorKind::NotFound,
            );

            let path = PathAbs::new(raw_path, "").unwrap();

            assert_eq!(path::Path::as_os_str(path.as_path()), "/does/not/exist");
        }

        #[test]
        fn absolute_path_cannot_go_above_root() {
            setup();
            let err = PathAbs::new("/foo/../..", "").unwrap_err();

            assert_eq!(err.io_error().kind(), io::ErrorKind::NotFound);
            assert_eq!(err.io_error().to_string(), ".. consumed root");
            assert_eq!(err.action(), "resolving absolute");
            assert_eq!(err.path(), path::Path::new("/foo/../.."));
        }
    }

    #[cfg(windows)]
    mod windows {
        use super::*;

        #[test]
        fn absolute_path_need_not_exist() {
            setup();

            // It's not likely this path would exist, but let's be sure.
            let raw_path = path::Path::new(r"C:\does\not\exist");
            assert_eq!(
                raw_path.metadata().unwrap_err().kind(),
                io::ErrorKind::NotFound,
            );

            let path = PathAbs::new(raw_path, "").unwrap();
            assert_eq!(
                path::Path::as_os_str(path.as_path()),
                r"\\?\C:\does\not\exist"
            );
        }

        #[test]
        fn absolute_path_cannot_go_above_root() {
            setup();
            let err = PathAbs::new(r"C:\foo\..\..", "").unwrap_err();

            assert_eq!(err.io_error().kind(), io::ErrorKind::NotFound);
            assert_eq!(err.io_error().to_string(), ".. consumed root");
            assert_eq!(err.action(), "resolving absolute");
            assert_eq!(err.path(), path::Path::new(r"C:\foo\..\.."));
        }

        #[test]
        fn absolute_supports_root_only_relative_path() {
            setup();
            let actual = PathAbs::new(r"\foo", "").unwrap();

            let mut current_drive_root = path::PathBuf::new();
            current_drive_root.extend(
                env::current_dir().unwrap().components().take(2), // the prefix (C:) and root (\) components
            );

            let expected = PathAbs::new(current_drive_root.join("foo"), "").unwrap();

            assert_eq!(actual, expected);
        }

        #[test]
        fn absolute_supports_prefix_only_relative_path() {
            setup();
            let actual = PathAbs::new(r"C:foo", "").unwrap();

            let expected = PathAbs::new(
                path::Path::new(r"C:").canonicalize().unwrap().join("foo"),
                "",
            )
            .unwrap();

            assert_eq!(actual, expected);
        }

        #[test]
        fn absolute_accepts_bogus_prefix() {
            setup();
            let path = PathAbs::new(r"\\?\bogus\path\", "").unwrap();

            assert_eq!(path::Path::as_os_str(path.as_path()), r"\\?\bogus\path");
        }
    }

    #[test]
    fn test_root_parent() {
        let actual = PathAbs::new("/a/../..", "").expect_err("Can go outside of `/`?");
        assert_eq!(actual.io_error().kind(), io::ErrorKind::NotFound);
        assert_eq!(actual.action(), "resolving absolute");
        assert_eq!(actual.path(), path::Path::new(r"/a/../.."));
    }
}
