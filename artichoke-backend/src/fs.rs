//! [`Artichoke`] virtual filesystem used for storing Ruby sources.

use artichoke_vfs::{FakeFileSystem, FileSystem};
use std::path::{Component, Path, PathBuf};

use crate::{Artichoke, ArtichokeError};

pub const RUBY_LOAD_PATH: &str = "/src/lib";

pub type RequireFunc = fn(&mut Artichoke) -> Result<(), ArtichokeError>;

/// Virtual filesystem that wraps a [`artichoke_vfs`] [`FakeFileSystem`].
pub struct Filesystem {
    fs: FakeFileSystem<Metadata>,
}

impl Filesystem {
    /// Create a new in memory virtual filesystem.
    ///
    /// Creates a directory at [`RUBY_LOAD_PATH`] for storing Ruby source files.
    /// This path is searched by
    /// [`Kernel::require`](crate::extn::core::kernel::Kernel::require) and
    /// [`Kernel::require_relative`](crate::extn::core::kernel::Kernel::require_relative).
    pub fn new() -> Result<Self, ArtichokeError> {
        let fs = FakeFileSystem::new();
        fs.create_dir_all(RUBY_LOAD_PATH)
            .map_err(ArtichokeError::Vfs)?;
        Ok(Self { fs })
    }

    pub fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<(), ArtichokeError> {
        let cwd = self.fs.current_dir().map_err(ArtichokeError::Vfs)?;
        let path = absolutize_relative_to(path, cwd);
        self.fs.create_dir_all(path).map_err(ArtichokeError::Vfs)
    }

    pub fn is_file<P: AsRef<Path>>(&self, path: P) -> bool {
        if let Ok(cwd) = self.fs.current_dir() {
            let path = absolutize_relative_to(path, cwd);
            self.fs.is_file(path)
        } else {
            false
        }
    }

    pub fn read_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>, ArtichokeError> {
        let cwd = self.fs.current_dir().map_err(ArtichokeError::Vfs)?;
        let path = absolutize_relative_to(path, cwd);
        self.fs.read_file(path).map_err(ArtichokeError::Vfs)
    }

    pub fn write_file<P, B>(&self, path: P, buf: B) -> Result<(), ArtichokeError>
    where
        P: AsRef<Path>,
        B: AsRef<[u8]>,
    {
        let cwd = self.fs.current_dir().map_err(ArtichokeError::Vfs)?;
        let path = absolutize_relative_to(path, cwd);
        self.fs.write_file(path, buf).map_err(ArtichokeError::Vfs)
    }

    pub fn set_metadata<P: AsRef<Path>>(
        &self,
        path: P,
        metadata: Metadata,
    ) -> Result<(), ArtichokeError> {
        let cwd = self.fs.current_dir().map_err(ArtichokeError::Vfs)?;
        let path = absolutize_relative_to(path, cwd);
        self.fs
            .set_metadata(path, metadata)
            .map_err(ArtichokeError::Vfs)
    }

    pub fn metadata<P: AsRef<Path>>(&self, path: P) -> Option<Metadata> {
        let cwd = self.fs.current_dir().ok()?;
        let path = absolutize_relative_to(path, cwd);
        self.fs.metadata(path)
    }
}

#[derive(Clone)]
pub struct Metadata {
    pub require: Option<RequireFunc>,
    already_required: bool,
}

impl Metadata {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn mark_required(self) -> Self {
        Self {
            require: self.require,
            already_required: true,
        }
    }

    #[must_use]
    pub fn is_already_required(&self) -> bool {
        self.already_required
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            require: None,
            already_required: false,
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
