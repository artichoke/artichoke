//! Virtual filesystem.
//!
//! Artichoke proxies all filesystem access through a
//! [virtual filesystem](Virtual). The filesystem can store Ruby sources and
//! [extension hooks](ExtensionHook) in memory and will support proxying to the
//! host filesystem for reads and writes.
//!
//! Artichoke uses the virtual filesystem to track metadata about loaded
//! features.

use std::borrow::Cow;
use std::collections::hash_map::Entry as HashEntry;
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::path::{Component, Path, PathBuf};

use crate::exception::Exception;
use crate::Artichoke;

/// Directory at which Ruby sources and extensions are stored in the virtual
/// filesystem.
///
/// `RUBY_LOAD_PATH` is the default current working directory for [`Virtual`]
/// filesystems.
pub const RUBY_LOAD_PATH: &str = "/src/lib";

/// Function type for extension hooks stored in the virtual filesystem.
///
/// This signature is equivalent to the signature for `File::require` as defined
/// by the `artichoke-backend` implementation of
/// [`LoadSources`](crate::LoadSources).
pub type ExtensionHook = fn(&mut Artichoke) -> Result<(), Exception>;

#[cfg(test)]
mod hook_prototype_tests {
    use crate::test::prelude::*;

    struct TestFile;

    impl File for TestFile {
        type Artichoke = Artichoke;
        type Error = Exception;

        fn require(_interp: &mut Artichoke) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[test]
    fn prototype() {
        // must compile
        let _ = super::Extension::new(TestFile::require);
    }
}

struct Extension {
    hook: ExtensionHook,
}

impl Extension {
    fn new(hook: ExtensionHook) -> Self {
        Self { hook }
    }
}

impl fmt::Debug for Extension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Extension")
            .field("hook", &"fn(&mut Artichoke) -> Result<(), Exception>")
            .finish()
    }
}

#[derive(Debug)]
struct Code {
    content: Cow<'static, [u8]>,
}

impl Code {
    fn new<T>(content: T) -> Self
    where
        T: Into<Cow<'static, [u8]>>,
    {
        let content = content.into();
        Self { content }
    }
}

#[derive(Debug)]
struct Entry {
    code: Option<Code>,
    extension: Option<Extension>,
    required: bool,
}

impl Entry {
    fn default_file_contents() -> &'static [u8] {
        &b"# virtual source file"[..]
    }

    fn from_code<T>(content: T) -> Self
    where
        T: Into<Cow<'static, [u8]>>,
    {
        Self {
            code: Some(Code::new(content.into())),
            extension: None,
            required: false,
        }
    }

    fn from_ext(hook: ExtensionHook) -> Self {
        Self {
            code: None,
            extension: Some(Extension::new(hook)),
            required: false,
        }
    }

    fn replace_content<T>(&mut self, content: T)
    where
        T: Into<Cow<'static, [u8]>>,
    {
        self.code.replace(Code::new(content.into()));
    }

    fn set_extension(&mut self, hook: ExtensionHook) {
        self.extension.replace(Extension::new(hook));
    }

    fn extension(&self) -> Option<ExtensionHook> {
        self.extension.as_ref().map(|ext| ext.hook)
    }

    fn is_required(&self) -> bool {
        self.required
    }

    fn mark_required(&mut self) {
        self.required = true;
    }
}

/// Virtual filesystem for sources, extensions, and require metadata.
///
/// `Virtual` is a [`HashMap`] from paths to an entry struct that contains:
///
/// - A bit for whether the path that points to the entry has been required
///   before.
/// - Optional binary content representing Ruby source code.
/// - Optional hook to a Rust function to be executed on `require` (similar to a
///   MRI C extension rubygem).
///
/// Sources in `Virtual` are only writable via the
/// [`LoadSources`](crate::LoadSources) trait. Sources can only be completely
/// replaced.
///
/// These APIs are consumed primarily by the `Kernel::require` implementation in
/// [`extn::core::kernel::require`](crate::extn::core::kernel::require).
#[derive(Debug)]
pub struct Virtual {
    fs: HashMap<PathBuf, Entry>,
    cwd: PathBuf,
}

impl Default for Virtual {
    /// Virtual filesystem with current working directory set to
    /// [`RUBY_LOAD_PATH`].
    fn default() -> Self {
        Self {
            fs: HashMap::default(),
            cwd: PathBuf::from(RUBY_LOAD_PATH),
        }
    }
}

impl Virtual {
    /// Create a new in memory virtual filesystem.
    ///
    /// Sets the current working directory of the VFS to [`RUBY_LOAD_PATH`] for
    /// storing Ruby source files. This path is searched by
    /// [`Kernel::require`, `Kernel::require_relative`, and `Kernel::load`](crate::extn::core::kernel::require).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Check whether `path` points to a file in the virtual filesystem.
    ///
    /// This API is infallible and will return `false` for non-existent paths.
    pub fn is_file<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = absolutize_relative_to(path, &self.cwd);
        self.fs.contains_key(&path)
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
    pub fn read_file<P: AsRef<Path>>(&self, path: P) -> io::Result<&[u8]> {
        let path = absolutize_relative_to(path, &self.cwd);
        if let Some(ref entry) = self.fs.get(&path) {
            if let Some(ref code) = entry.code {
                Ok(code.content.as_ref())
            } else {
                Ok(Entry::default_file_contents())
            }
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "file not found in virtual file system",
            ))
        }
    }

    /// Write file contents into the virtual file system at `path`.
    ///
    /// Writes the full file contents. If any file contents already exist at
    /// `path`, they are replaced. Extension hooks are preserved.
    ///
    /// # Errors
    ///
    /// This API is currently infallible but returns [`io::Result`] to reserve
    /// the ability to return errors in the future.
    pub fn write_file<P, T>(&mut self, path: P, buf: T) -> io::Result<()>
    where
        P: AsRef<Path>,
        T: Into<Cow<'static, [u8]>>,
    {
        let path = absolutize_relative_to(path, &self.cwd);
        match self.fs.entry(path) {
            HashEntry::Occupied(mut entry) => {
                entry.get_mut().replace_content(buf.into());
            }
            HashEntry::Vacant(entry) => {
                entry.insert(Entry::from_code(buf.into()));
            }
        }
        Ok(())
    }

    /// Retrieve an extension hook for the file at `path`.
    ///
    /// This API is infallible and will return `None` for non-existent paths.
    pub fn get_extension<P: AsRef<Path>>(&self, path: P) -> Option<ExtensionHook> {
        let path = absolutize_relative_to(path, &self.cwd);
        if let Some(entry) = self.fs.get(&path) {
            entry.extension()
        } else {
            None
        }
    }

    /// Write extension hook into the virtual file system at `path`.
    ///
    /// If any extension hooks already exist at `path`, they are replaced. File
    /// contents are preserved.
    ///
    /// # Errors
    ///
    /// This API is currently infallible but returns [`io::Result`] to reserve
    /// the ability to return errors in the future.
    pub fn register_extension<P: AsRef<Path>>(
        &mut self,
        path: P,
        extension: ExtensionHook,
    ) -> io::Result<()> {
        let path = absolutize_relative_to(path, &self.cwd);
        match self.fs.entry(path) {
            HashEntry::Occupied(mut entry) => {
                entry.get_mut().set_extension(extension);
            }
            HashEntry::Vacant(entry) => {
                entry.insert(Entry::from_ext(extension));
            }
        }
        Ok(())
    }

    /// Check whether a file at `path` has been required already.
    ///
    /// This API is infallible and will return `false` for non-existent paths.
    pub fn is_required<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = absolutize_relative_to(path, &self.cwd);
        if let Some(entry) = self.fs.get(&path) {
            entry.is_required()
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
    pub fn mark_required<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let path = absolutize_relative_to(path, &self.cwd);
        if let Some(entry) = self.fs.get_mut(&path) {
            entry.mark_required();
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "file not found in virtual file system",
            ))
        }
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
        let mut components = cwd
            .as_ref()
            .components()
            .map(Component::as_os_str)
            .collect::<Vec<_>>();
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
        assert_eq!(
            absolutize_relative_to(&path, cwd),
            Path::new("/home/artichoke/foo/bar")
        );
        let cwd = Path::new("relative/path");
        assert_eq!(
            absolutize_relative_to(&path, cwd),
            Path::new("relative/path/foo/bar")
        );
    }

    #[test]
    fn absolutize_relative_path_dedot_current_dir() {
        let path = Path::new("././././foo/./bar/./././.");
        let cwd = Path::new("/home/artichoke");
        assert_eq!(
            absolutize_relative_to(&path, cwd),
            Path::new("/home/artichoke/foo/bar")
        );
        let cwd = Path::new("relative/path");
        assert_eq!(
            absolutize_relative_to(&path, cwd),
            Path::new("relative/path/foo/bar")
        );
    }

    #[test]
    // TODO(GH-528): fix failing tests on Windows.
    #[cfg_attr(target_os = "windows", should_panic)]
    fn absolutize_relative_path_dedot_parent_dir() {
        let path = Path::new("foo/bar/..");
        let cwd = Path::new("/home/artichoke");
        assert_eq!(
            absolutize_relative_to(&path, cwd),
            Path::new("/home/artichoke/foo")
        );
        let cwd = Path::new("relative/path");
        assert_eq!(
            absolutize_relative_to(&path, cwd),
            Path::new("relative/path/foo")
        );

        let path = Path::new("foo/../../../../bar/../../../");
        let cwd = Path::new("/home/artichoke");
        assert_eq!(absolutize_relative_to(&path, cwd), Path::new("/"));
        let cwd = Path::new("relative/path");
        assert_eq!(absolutize_relative_to(&path, cwd), Path::new(""));

        let path = Path::new("foo/../../../../bar/../../../boom/baz");
        let cwd = Path::new("/home/artichoke");
        assert_eq!(absolutize_relative_to(&path, cwd), Path::new("/boom/baz"));
        let cwd = Path::new("relative/path");
        assert_eq!(absolutize_relative_to(&path, cwd), Path::new("boom/baz"));
    }
}
