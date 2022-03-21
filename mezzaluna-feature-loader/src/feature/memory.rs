use std::path::{Path, PathBuf};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Feature {
    path: PathBuf,
}

impl Feature {
    pub fn with_path(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &Path {
        &*self.path
    }
}
