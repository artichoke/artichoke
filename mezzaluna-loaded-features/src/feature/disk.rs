use core::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use same_file::Handle;

#[derive(Debug)]
pub struct Feature {
    handle: Handle,
    path: PathBuf,
}

impl Feature {
    pub fn with_handle_and_path(handle: Handle, path: PathBuf) -> Self {
        Self { handle, path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Hash for Feature {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.handle.hash(state);
    }
}

impl PartialEq for Feature {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl Eq for Feature {}
