//! [`Artichoke`] virtual filesystem used for storing Ruby sources.

use std::borrow::Cow;
use std::collections::hash_map::Entry as HashEntry;
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::path::{Component, Path, PathBuf};

use crate::exception::Exception;
use crate::Artichoke;

pub const RUBY_LOAD_PATH: &str = "/src/lib";

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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
        Self {
            content: content.into(),
        }
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
            code: Some(Code::new(content)),
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
        self.code.replace(Code::new(content));
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

/// Virtual filesystem that wraps a [`artichoke_vfs`] [`FakeFileSystem`].
#[derive(Debug)]
pub struct Virtual {
    fs: HashMap<PathBuf, Entry>,
    cwd: PathBuf,
}

impl Default for Virtual {
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
    /// Creates a directory at [`RUBY_LOAD_PATH`] for storing Ruby source files.
    /// This path is searched by
    /// [`Kernel::require`](crate::extn::core::kernel::Kernel::require) and
    /// [`Kernel::require_relative`](crate::extn::core::kernel::Kernel::require_relative).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_file<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = absolutize_relative_to(path, &self.cwd);
        self.fs.contains_key(&path)
    }

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

    pub fn write_file<P, T>(&mut self, path: P, buf: T) -> io::Result<()>
    where
        P: AsRef<Path>,
        T: Into<Cow<'static, [u8]>>,
    {
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

    pub fn is_required<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = absolutize_relative_to(path, &self.cwd);
        if let Some(entry) = self.fs.get(&path) {
            entry.is_required()
        } else {
            false
        }
    }

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

    pub fn get_extension<P: AsRef<Path>>(&self, path: P) -> Option<ExtensionHook> {
        let path = absolutize_relative_to(path, &self.cwd);
        if let Some(entry) = self.fs.get(&path) {
            entry.extension()
        } else {
            None
        }
    }

    pub fn register_extension<P: AsRef<Path>>(&mut self, path: P, extension: ExtensionHook) {
        let path = absolutize_relative_to(path, &self.cwd);
        match self.fs.entry(path) {
            HashEntry::Occupied(mut entry) => {
                entry.get_mut().set_extension(extension);
            }
            HashEntry::Vacant(entry) => {
                entry.insert(Entry::from_ext(extension));
            }
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
    let mut components = if let Some(Component::RootDir) = iter.peek() {
        Vec::with_capacity(hint.1.unwrap_or(hint.0))
    } else {
        let mut components = cwd
            .as_ref()
            .components()
            .map(Component::as_os_str)
            .collect::<Vec<_>>();
        components.reserve(hint.1.unwrap_or(hint.0));
        components
    };
    for component in iter {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                components.pop();
            }
            c => {
                components.push(c.as_os_str());
            }
        }
    }
    components.into_iter().collect()
}
