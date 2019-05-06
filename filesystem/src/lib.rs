#[cfg(any(feature = "mock", test))]
extern crate pseudo;
#[cfg(feature = "temp")]
extern crate rand;
#[cfg(feature = "temp")]
extern crate tempdir;

use std::ffi::OsString;
use std::io::Result;
use std::path::{Path, PathBuf};

#[cfg(feature = "fake")]
pub use fake::{FakeFileSystem, FakeTempDir};
#[cfg(any(feature = "mock", test))]
pub use mock::{FakeError, MockFileSystem};
pub use os::OsFileSystem;
#[cfg(feature = "temp")]
pub use os::OsTempDir;

#[cfg(feature = "fake")]
mod fake;
#[cfg(any(feature = "mock", test))]
mod mock;
mod os;

/// Provides standard file system operations.
pub trait FileSystem {
    type DirEntry: DirEntry;
    type ReadDir: ReadDir<Self::DirEntry>;

    /// Returns the current working directory.
    /// This is based on [`std::env::current_dir`].
    ///
    /// [`std::env::current_dir`]: https://doc.rust-lang.org/std/env/fn.current_dir.html
    fn current_dir(&self) -> Result<PathBuf>;
    /// Updates the current working directory.
    /// This is based on [`std::env::set_current_dir`].
    ///
    /// [`std::env::set_current_dir`]: https://doc.rust-lang.org/std/env/fn.set_current_dir.html
    fn set_current_dir<P: AsRef<Path>>(&self, path: P) -> Result<()>;

    /// Determines whether the path exists and points to a directory.
    fn is_dir<P: AsRef<Path>>(&self, path: P) -> bool;
    /// Determines whether the path exists and points to a file.
    fn is_file<P: AsRef<Path>>(&self, path: P) -> bool;

    /// Creates a new directory.
    /// This is based on [`std::fs::create_dir`].
    ///
    /// [`std::fs::create_dir`]: https://doc.rust-lang.org/std/fs/fn.create_dir.html
    fn create_dir<P: AsRef<Path>>(&self, path: P) -> Result<()>;
    /// Recursively creates a directory and any missing parents.
    /// This is based on [`std::fs::create_dir`].
    ///
    /// [`std::fs::create_dir_all`]: https://doc.rust-lang.org/std/fs/fn.create_dir_all.html
    fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<()>;
    /// Removes an empty directory.
    /// This is based on [`std::fs::remove_dir`].
    ///
    /// [`std::fs::remove_dir`]: https://doc.rust-lang.org/std/fs/fn.remove_dir.html
    fn remove_dir<P: AsRef<Path>>(&self, path: P) -> Result<()>;
    /// Removes a directory and any child files or directories.
    /// This is based on [`std::fs::remove_dir_all`].
    ///
    /// [`std::fs::remove_dir_all`]: https://doc.rust-lang.org/std/fs/fn.remove_dir_all.html
    fn remove_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<()>;
    /// Returns an iterator over the entries in a directory.
    /// This is based on [`std::fs::read_dir`].
    ///
    /// [`std::fs::read_dir`]: https://doc.rust-lang.org/std/fs/fn.read_dir.html
    fn read_dir<P: AsRef<Path>>(&self, path: P) -> Result<Self::ReadDir>;

    /// Writes `buf` to a new file at `path`.
    ///
    /// # Errors
    ///
    /// * A file or directory already exists at `path`.
    /// * The parent directory of `path` does not exist.
    /// * Current user has insufficient permissions.
    fn create_file<P, B>(&self, path: P, buf: B) -> Result<()>
    where
        P: AsRef<Path>,
        B: AsRef<[u8]>;
    /// Writes `buf` to a new or existing file at `buf`.
    /// This will overwrite any contents that already exist.
    ///
    /// # Errors
    ///
    /// * The parent directory of `path` does not exist.
    /// * Current user has insufficient permissions.
    fn write_file<P, B>(&self, path: P, buf: B) -> Result<()>
    where
        P: AsRef<Path>,
        B: AsRef<[u8]>;
    /// Writes `buf` to an existing file at `buf`.
    /// This will overwrite any contents that already exist.
    ///
    /// # Errors
    ///
    /// * No file `file` does not exist.
    /// * The node at `file` is a directory.
    /// * Current user has insufficient permissions.
    fn overwrite_file<P, B>(&self, path: P, buf: B) -> Result<()>
    where
        P: AsRef<Path>,
        B: AsRef<[u8]>;
    /// Returns the contents of `path`.
    ///
    /// # Errors
    ///
    /// * `path` does not exist.
    /// * `path` is a directory.
    /// * Current user has insufficient permissions.
    fn read_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>>;
    /// Returns the contents of `path` as a string.
    ///
    /// # Errors
    ///
    /// * `path` does not exist.
    /// * `path` is a directory.
    /// * Current user has insufficient permissions.
    /// * Contents are not valid UTF-8
    fn read_file_to_string<P: AsRef<Path>>(&self, path: P) -> Result<String>;
    /// Writes the contents of `path` into the buffer. If successful, returns
    /// the number of bytes that were read.
    ///
    /// # Errors
    ///
    /// * `path` does not exist.
    /// * `path` is a directory.
    /// * Current user has insufficient permissions.
    fn read_file_into<P, B>(&self, path: P, buf: B) -> Result<usize>
    where
        P: AsRef<Path>,
        B: AsMut<Vec<u8>>;
    /// Removes the file at `path`.
    /// This is based on [`std::fs::remove_file`].
    ///
    /// [`std::fs::remove_file`]: https://doc.rust-lang.org/std/fs/fn.remove_file.html
    fn remove_file<P: AsRef<Path>>(&self, path: P) -> Result<()>;
    /// Copies the file at path `from` to the path `to`.
    /// This is based on [`std::fs::copy`].
    ///
    /// [`std::fs::copy`]: https://doc.rust-lang.org/std/fs/fn.copy.html
    fn copy_file<P, Q>(&self, from: P, to: Q) -> Result<()>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>;

    /// Renames a file or directory.
    /// If both `from` and `to` are files, `to` will be replaced.
    /// Based on [`std::fs::rename`].
    ///
    /// [`std::fs::rename`]: https://doc.rust-lang.org/std/fs/fn.rename.html
    fn rename<P, Q>(&self, from: P, to: Q) -> Result<()>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>;

    /// Returns `true` if `path` is a readonly file.
    ///
    /// # Errors
    ///
    /// * `path` does not exist.
    /// * Current user has insufficient permissions.
    fn readonly<P: AsRef<Path>>(&self, path: P) -> Result<bool>;
    /// Sets or unsets the readonly flag of `path`.
    ///
    /// # Errors
    ///
    /// * `path` does not exist.
    /// * Current user has insufficient permissions.
    fn set_readonly<P: AsRef<Path>>(&self, path: P, readonly: bool) -> Result<()>;

    /// Returns the length of the node at the path
    /// or 0 if the node does not exist.
    fn len<P: AsRef<Path>>(&self, path: P) -> u64;
}

#[cfg(unix)]
pub trait UnixFileSystem {
    /// Returns the current mode bits of `path`.
    ///
    /// # Errors
    ///
    /// * `path` does not exist.
    /// * Current user has insufficient permissions.
    fn mode<P: AsRef<Path>>(&self, path: P) -> Result<u32>;
    /// Sets the mode bits of `path`.
    ///
    /// # Errors
    ///
    /// * `path` does not exist.
    /// * Current user has insufficient permissions.
    fn set_mode<P: AsRef<Path>>(&self, path: P, mode: u32) -> Result<()>;
}

#[cfg(feature = "temp")]
/// Tracks a temporary directory that will be deleted once the struct goes out of scope.
pub trait TempDir {
    /// Returns the [`Path`] of the temporary directory.
    ///
    /// [`Path`]: https://doc.rust-lang.org/std/path/struct.Path.html
    fn path(&self) -> &Path;
}

#[cfg(feature = "temp")]
pub trait TempFileSystem {
    type TempDir: TempDir;

    /// Creates a new temporary directory.
    fn temp_dir<S: AsRef<str>>(&self, prefix: S) -> Result<Self::TempDir>;
}

pub trait DirEntry {
    fn file_name(&self) -> OsString;
    fn path(&self) -> PathBuf;
}

pub trait ReadDir<T: DirEntry>: Iterator<Item = Result<T>> {}
