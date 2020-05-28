use std::borrow::Cow;
use std::collections::hash_map::Entry as HashEntry;
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

use crate::fs::{absolutize_relative_to, ExtensionHook, Filesystem, RUBY_LOAD_PATH};

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
/// `Memory` is a [`HashMap`] from paths to an entry struct that contains:
///
/// - A bit for whether the path that points to the entry has been required
///   before.
/// - Optional binary content representing Ruby source code.
/// - Optional hook to a Rust function to be executed on `require` (similar to a
///   MRI C extension rubygem).
///
/// Sources in `Memory` are only writable via the
/// [`LoadSources`](crate::core::LoadSources) trait. Sources can only be
/// completely replaced.
///
/// These APIs are consumed primarily by the `Kernel::require` implementation in
/// [`extn::core::kernel::require`](crate::extn::core::kernel::require).
#[derive(Debug)]
pub struct Memory {
    fs: HashMap<PathBuf, Entry>,
    cwd: PathBuf,
}

impl Default for Memory {
    /// Virtual filesystem with current working directory set to
    /// [`RUBY_LOAD_PATH`].
    fn default() -> Self {
        Self {
            fs: HashMap::default(),
            cwd: PathBuf::from(RUBY_LOAD_PATH),
        }
    }
}

impl Memory {
    /// Create a new in memory virtual filesystem.
    ///
    /// Sets the current working directory of the VFS to [`RUBY_LOAD_PATH`] for
    /// storing Ruby source files. This path is searched by
    /// [`Kernel::require`, `Kernel::require_relative`, and `Kernel::load`](crate::extn::core::kernel::require).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new in memory virtual filesystem with the given working
    /// directory.
    ///
    #[must_use]
    pub fn with_working_directory<T>(cwd: T) -> Self
    where
        T: Into<PathBuf>,
    {
        Self {
            fs: HashMap::default(),
            cwd: cwd.into(),
        }
    }
}

impl Filesystem for Memory {
    /// Check whether `path` points to a file in the virtual filesystem.
    ///
    /// This API is infallible and will return `false` for non-existent paths.
    fn is_file(&self, path: &Path) -> bool {
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
    fn read_file(&self, path: &Path) -> io::Result<Cow<'_, [u8]>> {
        let path = absolutize_relative_to(path, &self.cwd);
        if let Some(ref entry) = self.fs.get(&path) {
            if let Some(ref code) = entry.code {
                match code.content {
                    Cow::Borrowed(content) => Ok(content.into()),
                    Cow::Owned(ref content) => Ok(content.clone().into()),
                }
            } else {
                Ok(Entry::default_file_contents().into())
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
    fn write_file(&mut self, path: &Path, buf: Cow<'static, [u8]>) -> io::Result<()> {
        let path = absolutize_relative_to(path, &self.cwd);
        match self.fs.entry(path) {
            HashEntry::Occupied(mut entry) => {
                entry.get_mut().replace_content(buf);
            }
            HashEntry::Vacant(entry) => {
                entry.insert(Entry::from_code(buf));
            }
        }
        Ok(())
    }

    /// Retrieve an extension hook for the file at `path`.
    ///
    /// This API is infallible and will return `None` for non-existent paths.
    fn get_extension(&self, path: &Path) -> Option<ExtensionHook> {
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
    fn register_extension(&mut self, path: &Path, extension: ExtensionHook) -> io::Result<()> {
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
    fn is_required(&self, path: &Path) -> bool {
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
    fn mark_required(&mut self, path: &Path) -> io::Result<()> {
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

#[cfg(test)]
mod hook_prototype_tests {
    use crate::test::prelude::*;

    use super::Extension;

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
        let _ = Extension::new(TestFile::require);
    }
}
